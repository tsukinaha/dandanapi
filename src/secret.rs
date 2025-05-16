use std::str::FromStr;

use crate::{
    Error,
    Result,
};

#[derive(Debug, Clone)]
pub struct RequestHeaderGenerator {
    x_appid: Box<str>,
    secret: Box<str>,
}

impl Default for RequestHeaderGenerator {
    fn default() -> Self {
        Self {
            x_appid: "".into(),
            secret: "".into(),
        }
    }
}

impl RequestHeaderGenerator {
    pub fn new(x_appid: String, secret_generator: SecretGenerator) -> Result<Self> {
        let Some(secret) = secret_generator.generate_plaintext() else {
            return Err(Error::SecretGenerationError(
                "Failed to generate secret".to_string(),
            ));
        };

        Ok(Self {
            x_appid: x_appid.into(),
            secret: secret.into(),
        })
    }

    pub fn header(&self, path: &str) -> Result<reqwest::header::HeaderMap> {
        let mut header = reqwest::header::HeaderMap::new();
        header.insert("x-appid", self.x_appid.parse().unwrap());
        header.insert(
            "x-timestamp",
            chrono::Utc::now().timestamp().to_string().parse().unwrap(),
        );
        header.insert(
            "x-signature",
            self.calculate_signature(path)?.parse().unwrap(),
        );
        Ok(header)
    }

    pub fn calculate_signature(&self, path: &str) -> Result<String> {
        use base64::prelude::*;
        use sha2::{
            Digest,
            Sha256,
        };

        let timestamp = chrono::Utc::now().timestamp();
        let sha256 = Sha256::digest(
            format!("{}{}{}{}", self.x_appid, timestamp, path, self.secret).as_bytes(),
        );
        Ok(BASE64_STANDARD.encode(sha256))
    }
}

pub struct SecretGenerator {
    ciphertext: Vec<u8>,
    key: String,
}

impl SecretGenerator {
    pub fn new(ciphertext: Vec<u8>, key: String) -> Self {
        Self { ciphertext, key }
    }

    pub fn generate_plaintext(&self) -> Option<String> {
        let key = age::x25519::Identity::from_str(&self.key).ok()?;
        let pl = age::decrypt(&key, &self.ciphertext).ok()?;
        String::from_utf8(pl).ok()
    }
}

#[cfg(test)]
mod tests {
    use age::secrecy::ExposeSecret;

    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_generate_plaintext() {
        let key = include_str!("../ed25519_key");
        let ciphertext = include_bytes!("../secret");
        let generator = SecretGenerator::new(ciphertext.to_vec(), key.to_string());
        let key = generator.generate_plaintext().unwrap();
        println!("key: {}", key);
    }
}
