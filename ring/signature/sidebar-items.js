initSidebarItems({"fn":[["verify","Verify the signature `signature` of message `msg` with the public key `public_key` using the algorithm `alg`."]],"mod":[["primitive","Lower-level verification primitives. Usage of `ring::signature::verify()` is preferred when the public key and signature are encoded in standard formats, as it also handles the parsing."]],"static":[["ECDSA_P256_SHA256_ASN1","Verification of ASN.1 DER-encoded ECDSA signatures using the P-256 curve and SHA-256."],["ECDSA_P256_SHA384_ASN1","*Not recommended*. Verification of ASN.1 DER-encoded ECDSA signatures using the P-256 curve and SHA-384."],["ECDSA_P384_SHA256_ASN1","*Not recommended*. Verification of ASN.1 DER-encoded ECDSA signatures using the P-384 curve and SHA-256."],["ECDSA_P384_SHA384_ASN1","Verification of ASN.1 DER-encoded ECDSA signatures using the P-384 curve and SHA-384."],["ED25519","Verification of [Ed25519] signatures."],["RSA_PKCS1_2048_8192_SHA1","Verification of signatures using RSA keys of 2048-8192 bits,              PKCS#1.5 padding, and SHA-1."],["RSA_PKCS1_2048_8192_SHA256","Verification of signatures using RSA keys of 2048-8192 bits,              PKCS#1.5 padding, and SHA-256."],["RSA_PKCS1_2048_8192_SHA384","Verification of signatures using RSA keys of 2048-8192 bits,              PKCS#1.5 padding, and SHA-384."],["RSA_PKCS1_2048_8192_SHA512","Verification of signatures using RSA keys of 2048-8192 bits,              PKCS#1.5 padding, and SHA-512."],["RSA_PKCS1_3072_8192_SHA384","Verification of signatures using RSA keys of 3072-8192 bits,              PKCS#1.5 padding, and SHA-384."],["RSA_PKCS1_SHA256","PKCS#1 1.5 padding using SHA-256 for RSA signatures. Feature: `rsa_signing`."],["RSA_PKCS1_SHA384","PKCS#1 1.5 padding using SHA-384 for RSA signatures. Feature: `rsa_signing`."],["RSA_PKCS1_SHA512","PKCS#1 1.5 padding using SHA-512 for RSA signatures. Feature: `rsa_signing`."],["RSA_PSS_2048_8192_SHA256","Verification of signatures using RSA keys of 2048-8192 bits,              PSS padding, and SHA-256."],["RSA_PSS_2048_8192_SHA384","Verification of signatures using RSA keys of 2048-8192 bits,              PSS padding, and SHA-384."],["RSA_PSS_2048_8192_SHA512","Verification of signatures using RSA keys of 2048-8192 bits,              PSS padding, and SHA-512."],["RSA_PSS_SHA256","RSA PSS padding using SHA-256 for RSA signatures."],["RSA_PSS_SHA384","RSA PSS padding using SHA-384 for RSA signatures."],["RSA_PSS_SHA512","RSA PSS padding using SHA-512 for RSA signatures."]],"struct":[["ECDSAParameters","Parameters for ECDSA signing and verification."],["Ed25519KeyPair","An Ed25519 key pair, for signing."],["Ed25519KeyPairBytes","The raw bytes of the Ed25519 key pair, for serialization."],["EdDSAParameters","Parameters for EdDSA signing and verification."],["RSAKeyPair","An RSA key pair, used for signing. Feature: `rsa_signing`."],["RSAParameters","Parameters for RSA verification."],["RSASigningState","State used for RSA Signing. Feature: `rsa_signing`."],["Signature","A public key signature returned from a signing operation."]],"trait":[["RSAEncoding","An RSA signature encoding as described in [RFC 3447 Section 8]."],["VerificationAlgorithm","A signature verification algorithm."]]});