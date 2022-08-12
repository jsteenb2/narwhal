(function() {var implementors = {
"consensus":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"dag/node_dag/enum.NodeDagError.html\" title=\"enum dag::node_dag::NodeDagError\">NodeDagError</a>&gt; for <a class=\"enum\" href=\"consensus/dag/enum.ValidatorDagError.html\" title=\"enum consensus::dag::ValidatorDagError\">ValidatorDagError</a>"]],
"crypto":[["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;'a <a class=\"struct\" href=\"crypto/bls12381/struct.BLS12381PrivateKey.html\" title=\"struct crypto::bls12381::BLS12381PrivateKey\">BLS12381PrivateKey</a>&gt; for <a class=\"struct\" href=\"crypto/bls12381/struct.BLS12381PublicKey.html\" title=\"struct crypto::bls12381::BLS12381PublicKey\">BLS12381PublicKey</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"crypto/bls12381/struct.BLS12381PrivateKey.html\" title=\"struct crypto::bls12381::BLS12381PrivateKey\">BLS12381PrivateKey</a>&gt; for <a class=\"struct\" href=\"crypto/bls12381/struct.BLS12381KeyPair.html\" title=\"struct crypto::bls12381::BLS12381KeyPair\">BLS12381KeyPair</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;<a class=\"struct\" href=\"crypto/bls12381/struct.BLS12381PublicKey.html\" title=\"struct crypto::bls12381::BLS12381PublicKey\">BLS12381PublicKey</a>&gt; for <a class=\"type\" href=\"crypto/bls12381/type.BLS12381PublicKeyBytes.html\" title=\"type crypto::bls12381::BLS12381PublicKeyBytes\">BLS12381PublicKeyBytes</a>"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;'a <a class=\"struct\" href=\"crypto/ed25519/struct.Ed25519PrivateKey.html\" title=\"struct crypto::ed25519::Ed25519PrivateKey\">Ed25519PrivateKey</a>&gt; for <a class=\"struct\" href=\"crypto/ed25519/struct.Ed25519PublicKey.html\" title=\"struct crypto::ed25519::Ed25519PublicKey\">Ed25519PublicKey</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"crypto/ed25519/struct.Ed25519PrivateKey.html\" title=\"struct crypto::ed25519::Ed25519PrivateKey\">Ed25519PrivateKey</a>&gt; for <a class=\"struct\" href=\"crypto/ed25519/struct.Ed25519KeyPair.html\" title=\"struct crypto::ed25519::Ed25519KeyPair\">Ed25519KeyPair</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;Keypair&gt; for <a class=\"struct\" href=\"crypto/ed25519/struct.Ed25519KeyPair.html\" title=\"struct crypto::ed25519::Ed25519KeyPair\">Ed25519KeyPair</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;<a class=\"struct\" href=\"crypto/ed25519/struct.Ed25519PublicKey.html\" title=\"struct crypto::ed25519::Ed25519PublicKey\">Ed25519PublicKey</a>&gt; for <a class=\"type\" href=\"crypto/ed25519/type.Ed25519PublicKeyBytes.html\" title=\"type crypto::ed25519::Ed25519PublicKeyBytes\">Ed25519PublicKeyBytes</a>"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;'a <a class=\"struct\" href=\"crypto/secp256k1/struct.Secp256k1PrivateKey.html\" title=\"struct crypto::secp256k1::Secp256k1PrivateKey\">Secp256k1PrivateKey</a>&gt; for <a class=\"struct\" href=\"crypto/secp256k1/struct.Secp256k1PublicKey.html\" title=\"struct crypto::secp256k1::Secp256k1PublicKey\">Secp256k1PublicKey</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;<a class=\"struct\" href=\"crypto/secp256k1/struct.Secp256k1PublicKey.html\" title=\"struct crypto::secp256k1::Secp256k1PublicKey\">Secp256k1PublicKey</a>&gt; for <a class=\"type\" href=\"crypto/secp256k1/type.Secp256k1PublicKeyBytes.html\" title=\"type crypto::secp256k1::Secp256k1PublicKeyBytes\">Secp256k1PublicKeyBytes</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"crypto/secp256k1/struct.Secp256k1PrivateKey.html\" title=\"struct crypto::secp256k1::Secp256k1PrivateKey\">Secp256k1PrivateKey</a>&gt; for <a class=\"struct\" href=\"crypto/secp256k1/struct.Secp256k1KeyPair.html\" title=\"struct crypto::secp256k1::Secp256k1KeyPair\">Secp256k1KeyPair</a>"]],
"dag":[["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html\" title=\"struct alloc::sync::Arc\">Arc</a>&lt;<a class=\"struct\" href=\"dag/struct.Node.html\" title=\"struct dag::Node\">Node</a>&lt;T&gt;&gt;&gt; for <a class=\"struct\" href=\"dag/struct.NodeRef.html\" title=\"struct dag::NodeRef\">NodeRef</a>&lt;T&gt;"],["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"dag/struct.Node.html\" title=\"struct dag::Node\">Node</a>&lt;T&gt;&gt; for <a class=\"struct\" href=\"dag/struct.NodeRef.html\" title=\"struct dag::NodeRef\">NodeRef</a>&lt;T&gt;"]],
"demo_client":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"demo_client/narwhal/collection_error/enum.CollectionErrorType.html\" title=\"enum demo_client::narwhal::collection_error::CollectionErrorType\">CollectionErrorType</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i32.html\">i32</a>"]],
"executor":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;TypedStoreError&gt; for <a class=\"enum\" href=\"executor/enum.SubscriberError.html\" title=\"enum executor::SubscriberError\">SubscriberError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html\" title=\"struct alloc::boxed::Box\">Box</a>&lt;<a class=\"enum\" href=\"https://docs.rs/bincode/1.3.3/bincode/error/enum.ErrorKind.html\" title=\"enum bincode::error::ErrorKind\">ErrorKind</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;&gt; for <a class=\"enum\" href=\"executor/enum.SubscriberError.html\" title=\"enum executor::SubscriberError\">SubscriberError</a>"],["impl&lt;T:&nbsp;<a class=\"trait\" href=\"executor/trait.ExecutionStateError.html\" title=\"trait executor::ExecutionStateError\">ExecutionStateError</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;T&gt; for <a class=\"enum\" href=\"executor/enum.SubscriberError.html\" title=\"enum executor::SubscriberError\">SubscriberError</a>"]],
"types":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://docs.rs/signature/1.5.0/signature/error/struct.Error.html\" title=\"struct signature::error::Error\">Error</a>&gt; for <a class=\"enum\" href=\"types/error/enum.DagError.html\" title=\"enum types::error::DagError\">DagError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;TypedStoreError&gt; for <a class=\"enum\" href=\"types/error/enum.DagError.html\" title=\"enum types::error::DagError\">DagError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html\" title=\"struct alloc::boxed::Box\">Box</a>&lt;<a class=\"enum\" href=\"https://docs.rs/bincode/1.3.3/bincode/error/enum.ErrorKind.html\" title=\"enum bincode::error::ErrorKind\">ErrorKind</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;&gt; for <a class=\"enum\" href=\"types/error/enum.DagError.html\" title=\"enum types::error::DagError\">DagError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"types/struct.BatchDigest.html\" title=\"struct types::BatchDigest\">BatchDigest</a>&gt; for <a class=\"struct\" href=\"crypto/struct.Digest.html\" title=\"struct crypto::Digest\">Digest</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;UninitializedFieldError&gt; for <a class=\"enum\" href=\"types/enum.HeaderBuilderError.html\" title=\"enum types::HeaderBuilderError\">HeaderBuilderError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt; for <a class=\"enum\" href=\"types/enum.HeaderBuilderError.html\" title=\"enum types::HeaderBuilderError\">HeaderBuilderError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"types/struct.HeaderDigest.html\" title=\"struct types::HeaderDigest\">HeaderDigest</a>&gt; for <a class=\"struct\" href=\"crypto/struct.Digest.html\" title=\"struct crypto::Digest\">Digest</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"types/struct.VoteDigest.html\" title=\"struct types::VoteDigest\">VoteDigest</a>&gt; for <a class=\"struct\" href=\"crypto/struct.Digest.html\" title=\"struct crypto::Digest\">Digest</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"types/struct.CertificateDigest.html\" title=\"struct types::CertificateDigest\">CertificateDigest</a>&gt; for <a class=\"struct\" href=\"crypto/struct.Digest.html\" title=\"struct crypto::Digest\">Digest</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"types/struct.CertificateDigest.html\" title=\"struct types::CertificateDigest\">CertificateDigest</a>&gt; for <a class=\"struct\" href=\"types/struct.CertificateDigestProto.html\" title=\"struct types::CertificateDigestProto\">CertificateDigestProto</a>"],["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"types/struct.BlockError.html\" title=\"struct types::BlockError\">BlockError</a>&gt; for <a class=\"type\" href=\"types/type.BlockResult.html\" title=\"type types::BlockResult\">BlockResult</a>&lt;T&gt;"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"types/enum.CollectionErrorType.html\" title=\"enum types::CollectionErrorType\">CollectionErrorType</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i32.html\">i32</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"crypto/ed25519/struct.Ed25519PublicKey.html\" title=\"struct crypto::ed25519::Ed25519PublicKey\">Ed25519PublicKey</a>&gt; for <a class=\"struct\" href=\"types/struct.PublicKeyProto.html\" title=\"struct types::PublicKeyProto\">PublicKeyProto</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;&gt; for <a class=\"struct\" href=\"types/struct.TransactionProto.html\" title=\"struct types::TransactionProto\">TransactionProto</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"types/struct.TransactionProto.html\" title=\"struct types::TransactionProto\">Transaction</a>&gt; for <a class=\"type\" href=\"types/type.Transaction.html\" title=\"type types::Transaction\">Transaction</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"types/struct.BlockError.html\" title=\"struct types::BlockError\">BlockError</a>&gt; for <a class=\"struct\" href=\"types/struct.CollectionError.html\" title=\"struct types::CollectionError\">CollectionError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"types/enum.BlockErrorKind.html\" title=\"enum types::BlockErrorKind\">BlockErrorKind</a>&gt; for <a class=\"enum\" href=\"types/enum.CollectionErrorType.html\" title=\"enum types::CollectionErrorType\">CollectionErrorType</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;Bytes&gt; for <a class=\"struct\" href=\"types/struct.BincodeEncodedPayload.html\" title=\"struct types::BincodeEncodedPayload\">BincodeEncodedPayload</a>"],["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"types/metered_channel/struct.Receiver.html\" title=\"struct types::metered_channel::Receiver\">Receiver</a>&lt;T&gt;&gt; for <a class=\"struct\" href=\"types/metered_channel/struct.ReceiverStream.html\" title=\"struct types::metered_channel::ReceiverStream\">ReceiverStream</a>&lt;T&gt;"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()