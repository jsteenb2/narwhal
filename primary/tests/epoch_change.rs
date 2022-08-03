// Copyright (c) 2022, Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
use arc_swap::ArcSwap;
use config::{Committee, Epoch, Parameters};
use crypto::traits::KeyPair;
use futures::future::join_all;
use network::{CancelOnDropHandler, ReliableNetwork, WorkerToPrimaryNetwork};
use node::NodeStorage;
use primary::{NetworkModel, Primary, CHANNEL_CAPACITY};
use prometheus::Registry;
use std::{collections::BTreeMap, sync::Arc};
use test_utils::{
    keys, make_authority, pure_committee_from_keys, temp_dir, worker_cache_from_keys,
};
use tokio::sync::watch;
use types::{ReconfigureNotification, WorkerPrimaryMessage};

/// The epoch changes but the stake distribution and network addresses stay the same.
#[tokio::test]
async fn test_simple_epoch_change() {
    let parameters = Parameters {
        header_size: 32, // One batch digest
        ..Parameters::default()
    };

    // The configuration of epoch 0.
    let keys_0 = keys(None);
    let committee_0 = pure_committee_from_keys(&keys_0);
    let worker_cache_0 = worker_cache_from_keys(&keys_0);

    // Spawn the committee of epoch 0.
    let mut rx_channels = Vec::new();
    let mut tx_channels = Vec::new();
    for keypair in keys_0 {
        let name = keypair.public().clone();
        let signer = keypair;

        let (tx_new_certificates, rx_new_certificates) =
            test_utils::test_new_certificates_channel!(CHANNEL_CAPACITY);
        rx_channels.push(rx_new_certificates);
        let (tx_feedback, rx_feedback) =
            test_utils::test_committed_certificates_channel!(CHANNEL_CAPACITY);
        tx_channels.push(tx_feedback.clone());

        let initial_committee = ReconfigureNotification::NewEpoch(committee_0.clone());
        let (tx_reconfigure, _rx_reconfigure) = watch::channel(initial_committee);
        let (tx_get_block_commands, rx_get_block_commands) =
            test_utils::test_get_block_commands!(1);

        let store = NodeStorage::reopen(temp_dir());

        Primary::spawn(
            name,
            signer,
            Arc::new(ArcSwap::from_pointee(committee_0.clone())),
            Arc::new(ArcSwap::from_pointee(worker_cache_0.clone())),
            parameters.clone(),
            store.header_store.clone(),
            store.certificate_store.clone(),
            store.payload_store.clone(),
            /* tx_consensus */ tx_new_certificates,
            /* rx_consensus */ rx_feedback,
            tx_get_block_commands,
            rx_get_block_commands,
            /* dag */ None,
            NetworkModel::Asynchronous,
            tx_reconfigure,
            /* tx_committed_certificates */ tx_feedback,
            &Registry::new(),
        );
    }

    // Run for a while in epoch 0.
    for rx in rx_channels.iter_mut() {
        loop {
            let certificate = rx.recv().await.unwrap();
            assert_eq!(certificate.epoch(), 0);
            if certificate.round() == 10 {
                break;
            }
        }
    }

    // Move to the next epochs.
    let mut old_committee = committee_0;
    for epoch in 1..=3 {
        // Move to the next epoch.
        let new_committee = Committee {
            epoch,
            ..old_committee.clone()
        };

        // Notify the old committee to change epoch.
        let addresses: Vec<_> = old_committee
            .authorities
            .values()
            .map(|authority| authority.primary.worker_to_primary.clone())
            .collect();
        let message = WorkerPrimaryMessage::Reconfigure(ReconfigureNotification::NewEpoch(
            new_committee.clone(),
        ));
        let mut _do_not_drop: Vec<CancelOnDropHandler<_>> = Vec::new();
        for address in addresses {
            _do_not_drop.push(
                WorkerToPrimaryNetwork::default()
                    .send(address, &message)
                    .await,
            );
        }

        // Run for a while.
        for rx in rx_channels.iter_mut() {
            loop {
                let certificate = rx.recv().await.unwrap();
                if certificate.epoch() == epoch && certificate.round() == 10 {
                    break;
                }
            }
        }

        old_committee = new_committee;
    }
}

#[tokio::test]
async fn test_partial_committee_change() {
    let parameters = Parameters {
        header_size: 32, // One batch digest
        ..Parameters::default()
    };

    // Make the committee of epoch 0.
    let keys_0 = keys(None);
    let authorities_0: Vec<_> = keys_0.iter().map(|_| make_authority()).collect();
    let committee_0 = Committee {
        epoch: Epoch::default(),
        authorities: keys_0
            .iter()
            .zip(authorities_0.clone().into_iter())
            .map(|(kp, authority)| (kp.public().clone(), authority))
            .collect(),
    };
    let worker_cache_0 = worker_cache_from_keys(&keys_0);

    // Spawn the committee of epoch 0.
    let mut epoch_0_rx_channels = Vec::new();
    let mut epoch_0_tx_channels = Vec::new();
    for keypair in keys_0 {
        let name = keypair.public().clone();
        let signer = keypair;

        let (tx_new_certificates, rx_new_certificates) =
            test_utils::test_new_certificates_channel!(CHANNEL_CAPACITY);
        epoch_0_rx_channels.push(rx_new_certificates);
        let (tx_feedback, rx_feedback) =
            test_utils::test_committed_certificates_channel!(CHANNEL_CAPACITY);
        epoch_0_tx_channels.push(tx_feedback.clone());
        let initial_committee = ReconfigureNotification::NewEpoch(committee_0.clone());
        let (tx_reconfigure, _rx_reconfigure) = watch::channel(initial_committee);
        let (tx_get_block_commands, rx_get_block_commands) =
            test_utils::test_get_block_commands!(1);

        let store = NodeStorage::reopen(temp_dir());

        Primary::spawn(
            name,
            signer,
            Arc::new(ArcSwap::from_pointee(committee_0.clone())),
            Arc::new(ArcSwap::from_pointee(worker_cache_0.clone())),
            parameters.clone(),
            store.header_store.clone(),
            store.certificate_store.clone(),
            store.payload_store.clone(),
            /* tx_consensus */ tx_new_certificates,
            /* rx_consensus */ rx_feedback,
            tx_get_block_commands,
            rx_get_block_commands,
            /* dag */ None,
            NetworkModel::Asynchronous,
            tx_reconfigure,
            /* tx_committed_certificates */ tx_feedback,
            &Registry::new(),
        );
    }

    // Run for a while in epoch 0.
    for rx in epoch_0_rx_channels.iter_mut() {
        loop {
            let certificate = rx.recv().await.unwrap();
            assert_eq!(certificate.epoch(), 0);
            if certificate.round() == 10 {
                break;
            }
        }
    }

    // Make the committee of epoch 1.
    let mut to_spawn = Vec::new();

    let keys_0 = keys(None);
    let keys_1 = keys(Some(1));
    let mut total_stake = 0;
    let mut committee_keys = vec![];
    let authorities_1: BTreeMap<_, _> = authorities_0
        .into_iter()
        .zip(keys_0.into_iter())
        .zip(keys_1.into_iter())
        .map(|((authority, key_0), key_1)| {
            let stake = authority.stake;
            let x = if total_stake < committee_0.validity_threshold() {
                let pk = key_0.public().clone();
                committee_keys.push(key_0);
                (pk, authority)
            } else {
                let new_authority = make_authority();
                let pk = key_1.public().clone();
                committee_keys.push(key_1.copy());
                to_spawn.push(key_1);
                (pk, new_authority)
            };
            total_stake += stake;
            x
        })
        .collect();

    let committee_1 = Committee {
        epoch: Epoch::default() + 1,
        authorities: authorities_1,
    };
    let worker_cache_1 = worker_cache_from_keys(&committee_keys);

    // Spawn the committee of epoch 1 (only the node not already booted).
    let mut epoch_1_rx_channels = Vec::new();
    let mut epoch_1_tx_channels = Vec::new();
    for keypair in to_spawn {
        let name = keypair.public().clone();
        let signer = keypair;

        let (tx_new_certificates, rx_new_certificates) =
            test_utils::test_new_certificates_channel!(CHANNEL_CAPACITY);
        epoch_1_rx_channels.push(rx_new_certificates);
        let (tx_feedback, rx_feedback) =
            test_utils::test_committed_certificates_channel!(CHANNEL_CAPACITY);
        epoch_1_tx_channels.push(tx_feedback.clone());
        let (tx_get_block_commands, rx_get_block_commands) =
            test_utils::test_get_block_commands!(1);

        let initial_committee = ReconfigureNotification::NewEpoch(committee_1.clone());
        let (tx_reconfigure, _rx_reconfigure) = watch::channel(initial_committee);

        let store = NodeStorage::reopen(temp_dir());

        Primary::spawn(
            name,
            signer,
            Arc::new(ArcSwap::from_pointee(committee_1.clone())),
            Arc::new(ArcSwap::from_pointee(worker_cache_1.clone())),
            parameters.clone(),
            store.header_store.clone(),
            store.certificate_store.clone(),
            store.payload_store.clone(),
            /* tx_consensus */ tx_new_certificates,
            /* rx_consensus */ rx_feedback,
            tx_get_block_commands,
            rx_get_block_commands,
            /* dag */ None,
            NetworkModel::Asynchronous,
            tx_reconfigure,
            /* tx_committed_certificates */ tx_feedback,
            &Registry::new(),
        );
    }

    // Tell the nodes of epoch 0 to transition to epoch 1.
    let addresses: Vec<_> = committee_0
        .authorities
        .values()
        .map(|authority| authority.primary.worker_to_primary.clone())
        .collect();
    let message =
        WorkerPrimaryMessage::Reconfigure(ReconfigureNotification::NewEpoch(committee_1.clone()));
    let mut _do_not_drop: Vec<CancelOnDropHandler<_>> = Vec::new();
    for address in addresses {
        _do_not_drop.push(
            WorkerToPrimaryNetwork::default()
                .send(address, &message)
                .await,
        );
    }

    // Run for a while in epoch 1.
    for rx in epoch_1_rx_channels.iter_mut() {
        loop {
            let certificate = rx.recv().await.unwrap();
            if certificate.epoch() == 1 && certificate.round() == 10 {
                break;
            }
        }
    }
}

/// The epoch changes but the stake distribution and network addresses stay the same.
#[tokio::test]
async fn test_restart_with_new_committee_change() {
    let parameters = Parameters {
        header_size: 32, // One batch digest
        ..Parameters::default()
    };

    // The configuration of epoch 0.
    let keys_0 = keys(None);
    let committee_0 = pure_committee_from_keys(&keys_0);
    let worker_cache_0 = worker_cache_from_keys(&keys_0);

    // Spawn the committee of epoch 0.
    let mut rx_channels = Vec::new();
    let mut tx_channels = Vec::new();
    let mut handles = Vec::new();
    for keypair in keys_0 {
        let name = keypair.public().clone();
        let signer = keypair;

        let (tx_new_certificates, rx_new_certificates) =
            test_utils::test_new_certificates_channel!(CHANNEL_CAPACITY);
        rx_channels.push(rx_new_certificates);
        let (tx_feedback, rx_feedback) =
            test_utils::test_committed_certificates_channel!(CHANNEL_CAPACITY);
        tx_channels.push(tx_feedback.clone());
        let (tx_get_block_commands, rx_get_block_commands) =
            test_utils::test_get_block_commands!(1);

        let initial_committee = ReconfigureNotification::NewEpoch(committee_0.clone());
        let (tx_reconfigure, _rx_reconfigure) = watch::channel(initial_committee);

        let store = NodeStorage::reopen(temp_dir());
        let registry = Registry::new();
        let primary_handles = Primary::spawn(
            name,
            signer,
            Arc::new(ArcSwap::new(Arc::new(committee_0.clone()))),
            Arc::new(ArcSwap::new(Arc::new(worker_cache_0.clone()))),
            parameters.clone(),
            store.header_store.clone(),
            store.certificate_store.clone(),
            store.payload_store.clone(),
            /* tx_consensus */ tx_new_certificates,
            /* rx_consensus */ rx_feedback,
            tx_get_block_commands,
            rx_get_block_commands,
            /* dag */ None,
            NetworkModel::Asynchronous,
            tx_reconfigure,
            /* tx_committed_certificates */ tx_feedback,
            &registry,
        );
        handles.extend(primary_handles.into_iter().map(|(_n, j)| j));
    }

    // Run for a while in epoch 0.
    for rx in rx_channels.iter_mut() {
        loop {
            let certificate = rx.recv().await.unwrap();
            assert_eq!(certificate.epoch(), 0);
            if certificate.round() == 10 {
                break;
            }
        }
    }

    // Shutdown the committee of the previous epoch;
    let addresses: Vec<_> = committee_0
        .authorities
        .values()
        .map(|authority| authority.primary.worker_to_primary.clone())
        .collect();
    let message = WorkerPrimaryMessage::Reconfigure(ReconfigureNotification::Shutdown);
    let mut _do_not_drop: Vec<CancelOnDropHandler<_>> = Vec::new();
    for address in addresses {
        _do_not_drop.push(
            WorkerToPrimaryNetwork::default()
                .send(address, &message)
                .await,
        );
    }

    // Wait for the committee to shutdown.
    join_all(handles).await;

    // Move to the next epochs.
    for epoch in 1..=3 {
        let mut new_committee = committee_0.clone();
        new_committee.epoch = epoch;
        let mut new_worker_cache = worker_cache_0.clone();
        new_worker_cache.epoch = epoch;

        let mut rx_channels = Vec::new();
        let mut tx_channels = Vec::new();
        let mut handles = Vec::new();
        for keypair in keys(None) {
            let name = keypair.public().clone();
            let signer = keypair;

            let (tx_new_certificates, rx_new_certificates) =
                test_utils::test_channel!(CHANNEL_CAPACITY);
            rx_channels.push(rx_new_certificates);
            let (tx_feedback, rx_feedback) =
                test_utils::test_committed_certificates_channel!(CHANNEL_CAPACITY);
            tx_channels.push(tx_feedback.clone());

            let initial_committee = ReconfigureNotification::NewEpoch(new_committee.clone());
            let (tx_reconfigure, _rx_reconfigure) = watch::channel(initial_committee);
            let (tx_get_block_commands, rx_get_block_commands) =
                test_utils::test_get_block_commands!(1);

            let store = NodeStorage::reopen(temp_dir());
            let registry = Registry::new();
            let primary_handles = Primary::spawn(
                name,
                signer,
                Arc::new(ArcSwap::new(Arc::new(new_committee.clone()))),
                Arc::new(ArcSwap::new(Arc::new(new_worker_cache.clone()))),
                parameters.clone(),
                store.header_store.clone(),
                store.certificate_store.clone(),
                store.payload_store.clone(),
                /* tx_consensus */ tx_new_certificates,
                /* rx_consensus */ rx_feedback,
                tx_get_block_commands,
                rx_get_block_commands,
                /* dag */ None,
                NetworkModel::Asynchronous,
                tx_reconfigure,
                /* tx_committed_certificates */ tx_feedback,
                &registry,
            );
            handles.extend(primary_handles.into_iter().map(|(_n, j)| j));
        }

        // Run for a while.
        for rx in rx_channels.iter_mut() {
            loop {
                let certificate = rx.recv().await.unwrap();
                if certificate.epoch() == epoch && certificate.round() == 10 {
                    break;
                }
            }
        }

        // Shutdown the committee of the previous epoch;
        let addresses: Vec<_> = committee_0
            .authorities
            .values()
            .map(|authority| authority.primary.worker_to_primary.clone())
            .collect();
        let message = WorkerPrimaryMessage::Reconfigure(ReconfigureNotification::Shutdown);
        let mut _do_not_drop: Vec<CancelOnDropHandler<_>> = Vec::new();
        for address in addresses {
            _do_not_drop.push(
                WorkerToPrimaryNetwork::default()
                    .send(address, &message)
                    .await,
            );
        }

        // Wait for the committee to shutdown.
        join_all(handles).await;
    }
}

/// Update the committee without changing the epoch.
#[tokio::test]
async fn test_simple_committee_update() {
    let parameters = Parameters {
        header_size: 32, // One batch digest
        ..Parameters::default()
    };

    // The configuration of epoch 0.
    let keys_0 = keys(None);
    let committee_0 = pure_committee_from_keys(&keys_0);
    let worker_cache_0 = worker_cache_from_keys(&keys_0);

    // Spawn the committee of epoch 0.
    let mut rx_channels = Vec::new();
    let mut tx_channels = Vec::new();
    for keypair in keys_0 {
        let name = keypair.public().clone();
        let signer = keypair;

        let (tx_new_certificates, rx_new_certificates) =
            test_utils::test_channel!(CHANNEL_CAPACITY);
        rx_channels.push(rx_new_certificates);
        let (tx_feedback, rx_feedback) =
            test_utils::test_committed_certificates_channel!(CHANNEL_CAPACITY);
        tx_channels.push(tx_feedback.clone());

        let initial_committee = ReconfigureNotification::NewEpoch(committee_0.clone());
        let (tx_reconfigure, _rx_reconfigure) = watch::channel(initial_committee);
        let (tx_get_block_commands, rx_get_block_commands) =
            test_utils::test_get_block_commands!(1);

        let store = NodeStorage::reopen(temp_dir());

        Primary::spawn(
            name,
            signer,
            Arc::new(ArcSwap::from_pointee(committee_0.clone())),
            Arc::new(ArcSwap::from_pointee(worker_cache_0.clone())),
            parameters.clone(),
            store.header_store.clone(),
            store.certificate_store.clone(),
            store.payload_store.clone(),
            /* tx_consensus */ tx_new_certificates,
            /* rx_consensus */ rx_feedback,
            tx_get_block_commands,
            rx_get_block_commands,
            /* dag */ None,
            NetworkModel::Asynchronous,
            tx_reconfigure,
            /* tx_committed_certificates */ tx_feedback,
            &Registry::new(),
        );
    }

    // Run for a while in epoch 0.
    for rx in rx_channels.iter_mut() {
        loop {
            let certificate = rx.recv().await.unwrap();
            assert_eq!(certificate.epoch(), 0);
            if certificate.round() == 10 {
                break;
            }
        }
    }

    // Update the committee
    let mut old_committee = committee_0;
    for _ in 1..=3 {
        // Update the committee
        let mut new_committee = old_committee.clone();

        let mut total_stake = 0;
        let threshold = new_committee.validity_threshold();
        for (_, authority) in new_committee.authorities.iter_mut() {
            if total_stake < threshold {
                authority.primary.primary_to_primary = format!(
                    "/ip4/127.0.0.1/tcp/{}/http",
                    config::utils::get_available_port()
                )
                .parse()
                .unwrap();

                total_stake += authority.stake;
            }
        }

        // Notify the old committee about the change in committee information.
        let addresses: Vec<_> = old_committee
            .authorities
            .values()
            .map(|authority| authority.primary.worker_to_primary.clone())
            .collect();
        let message = WorkerPrimaryMessage::Reconfigure(ReconfigureNotification::UpdateCommittee(
            new_committee.clone(),
        ));
        let mut _do_not_drop: Vec<CancelOnDropHandler<_>> = Vec::new();
        for address in addresses {
            _do_not_drop.push(
                WorkerToPrimaryNetwork::default()
                    .send(address, &message)
                    .await,
            );
        }

        // Run for a while.
        for rx in rx_channels.iter_mut() {
            loop {
                let certificate = rx.recv().await.unwrap();
                assert_eq!(certificate.epoch(), 0);
                if certificate.round() == 10 {
                    break;
                }
            }
        }

        old_committee = new_committee;
    }
}
