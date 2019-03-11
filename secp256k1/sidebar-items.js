initSidebarItems({"enum":[["Error","An ECDSA error"]],"mod":[["constants","Constants Constants related to the API and the underlying curve"],["ecdh","ECDH Support for shared secret computations"],["ffi","FFI bindings Direct bindings to the underlying C library functions. These should not be needed for most users."],["key","Public and secret keys"]],"struct":[["All","Represents the set of all capabilities."],["Message","A (hashed) message input to an ECDSA signature"],["RecoverableSignature","An ECDSA signature with a recovery ID for pubkey recovery"],["RecoveryId","A tag used for recovering the public key from a compact signature"],["Secp256k1","The secp256k1 engine, used to execute all signature operations"],["SignOnly","Represents the set of capabilities needed for signing."],["Signature","An ECDSA signature"],["VerifyOnly","Represents the set of capabilities needed for verification."]],"trait":[["Signing","Marker trait for indicating that an instance of `Secp256k1` can be used for signing."],["ThirtyTwoByteHash","Trait describing something that promises to be a 32-byte random number; in particular, it has negligible probability of being zero or overflowing the group order. Such objects may be converted to `Message`s without any error paths."],["Verification","Marker trait for indicating that an instance of `Secp256k1` can be used for verification."]]});