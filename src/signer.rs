use time::get_time;
use std::io::Read;
use std::sync::Arc;
use serde_json;
use openssl::ec::EcKey;
use openssl::pkey::{PKey, Private};
use openssl::hash::MessageDigest;
use openssl::sign::Signer as SslSigner;
use base64::encode;
use crate::error::Error;
use crossbeam::sync::ArcCell;

#[derive(Debug)]
struct Signature {
    key: String,
    issued_at: i64,
}

/// For signing requests when using token-based authentication. Re-uses the same
/// signature for a certain amount of time.
pub struct Signer {
    signature: ArcCell<Signature>,
    key_id: String,
    team_id: String,
    secret: PKey<Private>,
    expire_after_s: i64,
}

#[derive(Serialize, Deserialize)]
enum JwtAlg {
    ES256,
}

#[derive(Serialize, Deserialize)]
struct JwtHeader<'a> {
    alg: JwtAlg,
    kid: &'a str,
}

#[derive(Serialize, Deserialize)]
struct JwtPayload<'a> {
    iss: &'a str,
    iat: i64,
}

impl Signer {
    /// Creates a signer with a pkcs8 private key, APNs key id and team id.
    /// Can fail if the key is not valid or there is a problem with system OpenSSL.
    pub fn new<S, T, R>(
        mut pk_pem: R,
        key_id: S,
        team_id: T,
        signature_ttl: i64,
    ) -> Result<Signer, Error>
    where
        S: Into<String>,
        T: Into<String>,
        R: Read,
    {
        let key_id: String = key_id.into();
        let team_id: String = team_id.into();

        let mut pem_key: Vec<u8> = Vec::new();
        pk_pem.read_to_end(&mut pem_key)?;

        let ec_key = EcKey::private_key_from_pem(&pem_key)?;

        let issued_at = get_time().sec;
        let secret = PKey::from_ec_key(ec_key)?;

        let signature = ArcCell::new(Arc::new(Signature {
            key: Self::create_signature(&secret, &key_id, &team_id, issued_at)?,
            issued_at: issued_at,
        }));

        let signer = Signer {
            signature: signature,
            key_id: key_id,
            team_id: team_id,
            secret: secret,
            expire_after_s: signature_ttl,
        };

        Ok(signer)
    }

    /// Take a signature out for usage. Automatically renews the signature
    /// if it's older than the expiration time.
    pub fn with_signature<F>(&self, f: F) -> Result<(), Error>
    where
        F: FnOnce(&str) -> (),
    {
        if self.is_expired() {
            self.renew()?;
        }

        let signature = self.signature.get();

        trace!(
            "Signer::with_signature found signature for {}/{} valid for {}s",
            self.key_id,
            self.team_id,
            self.expire_after_s - (get_time().sec - signature.issued_at)
        );

        Ok(f(&signature.key))
    }

    fn create_signature(
        secret: &PKey<Private>,
        key_id: &str,
        team_id: &str,
        issued_at: i64,
    ) -> Result<String, Error> {
        let headers = JwtHeader {
            alg: JwtAlg::ES256,
            kid: key_id,
        };

        let payload = JwtPayload {
            iss: team_id,
            iat: issued_at,
        };

        let encoded_header = encode(&serde_json::to_string(&headers)?);
        let encoded_payload = encode(&serde_json::to_string(&payload)?);
        let signing_input = format!("{}.{}", encoded_header, encoded_payload);

        let mut signer = SslSigner::new(MessageDigest::sha256(), secret)?;
        signer.update(signing_input.as_bytes())?;

        let signature_payload = signer.sign_to_vec()?;

        Ok(format!("{}.{}", signing_input, encode(&signature_payload)))
    }

    fn renew(&self) -> Result<(), Error> {
        let issued_at = get_time().sec;

        trace!(
            "Signer::renew for k_id {} t_id {} issued {} valid for {}s",
            self.key_id,
            self.team_id,
            issued_at,
            self.expire_after_s,
        );

        self.signature.set(Arc::new(Signature {
            key: Self::create_signature(&self.secret, &self.key_id, &self.team_id, issued_at)?,
            issued_at: issued_at,
        }));

        Ok(())
    }

    fn is_expired(&self) -> bool {
        let sig = self.signature.get();
        let expiry = get_time().sec - sig.issued_at;
        expiry >= self.expire_after_s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRIVATE_KEY: &'static str = indoc!(
        "-----BEGIN PRIVATE KEY-----
        MIGHAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBG0wawIBAQQg8g/n6j9roKvnUkwu
        lCEIvbDqlUhA5FOzcakkG90E8L+hRANCAATKS2ZExEybUvchRDuKBftotMwVEus3
        jDwmlD1Gg0yJt1e38djFwsxsfr5q2hv0Rj9fTEqAPr8H7mGm0wKxZ7iQ
        -----END PRIVATE KEY-----"
    );

    #[test]
    fn test_signature_caching() {
        let signer = Signer::new(PRIVATE_KEY.as_bytes(), "89AFRD1X22", "ASDFQWERTY", 100).unwrap();

        signer
            .with_signature(|sig1| {
                signer
                    .with_signature(|sig2| {
                        assert_eq!(sig1, sig2);
                    })
                    .unwrap();
            })
            .unwrap();
    }

    #[test]
    fn test_signature_without_caching() {
        let signer = Signer::new(PRIVATE_KEY.as_bytes(), "89AFRD1X22", "ASDFQWERTY", 0).unwrap();

        signer
            .with_signature(|sig1| {
                signer
                    .with_signature(|sig2| {
                        assert_ne!(sig1, sig2);
                    })
                    .unwrap();
            })
            .unwrap();
    }
}
