use std::time::SystemTime;
use rand::{distributions::Alphanumeric, Rng};

use crate::error::{Error, Result};

#[derive(Debug)]
pub struct AuthToken {
    user_id: i32,
    expiration: u64,
    signature: String,
}

// Parse a token of format "user-[user-id]-[expiration]-[signature]"
impl AuthToken {
    pub fn new(user_id: i32) -> Self {
        const COOKIE_EXPIRATION_TIME_IN_SECONDS: u64 = 60*60; // 1 hour
        let expiration = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() + COOKIE_EXPIRATION_TIME_IN_SECONDS;

        let signature = generate_signature();

        AuthToken{
            user_id,
            expiration,
            signature
        }
    }

    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        (self.expiration - now)>0
    }

    pub fn get_signature(&self) -> &str {
        &self.signature
    }

    pub fn from_str(token: &str) -> Result<Self> {
        let mut iter = token.split("-");
        let user_id = iter.nth(1).ok_or(Error::AuthTokenParsingError)?.parse().map_err(|_|Error::AuthTokenParsingError)?;
        let expiration = iter.next().ok_or(Error::AuthTokenParsingError)?.parse().map_err(|_|Error::AuthTokenParsingError)?;
        let signature = iter.collect::<Vec<&str>>().join("");

        Ok(AuthToken {
            user_id,
            expiration,
            signature
        })
    }

    pub fn to_str(&self) -> String {
        format!("user-{}-{}-{}", self.user_id, self.expiration, self.signature)
    }
}

fn generate_signature() -> String {
    let mut rng = rand::thread_rng();
    (0..20).map(|_| rng.sample(Alphanumeric) as char).collect()
}

#[test]
fn random_string_length() {
    let length = generate_signature().chars().count();
    assert_eq!(length, 20);
}