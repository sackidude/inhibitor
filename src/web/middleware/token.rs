use std::{alloc::System, time::SystemTime};

use rand::{distributions, Rng};

pub const AUTH_TOKEN: &str = "auth-token";

#[derive(Debug)]
pub struct AuthToken {
    user_id: i32,
    expiration: u64, // UNIX EPOCH TIME STAMP as u64
    signature: String,
}

impl AuthToken {
    pub fn new(user_id: i32) -> Self {
        const COOKIE_EXPIRATION_TIME_IN_SECONDS: u64 = 60*60; // 1 hour
        let now_in_secs = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        let expiration = now_in_secs + COOKIE_EXPIRATION_TIME_IN_SECONDS;

        let signature = generate_signature();

        AuthToken { user_id, expiration, signature }
    }

    pub fn from_str(token: &str) -> Option<Self> {
        let mut iter = token.split("-");
        let user_id = match iter.nth(1) {
            Some(id) => id.parse().unwrap(),
            None => return None,
        };
        let expiration = match iter.next() {
            Some(expiration) => expiration.parse().unwrap(),
            None => return None,
        };
        let signature = iter.collect::<Vec<&str>>().join("");

        Some(AuthToken {
            user_id,
            expiration,
            signature
        })
    }
}

impl AuthToken {
    pub fn to_str(&self) -> String {
        format!("user-{}-{}-{}", self.user_id, self.expiration, self.signature)
    }

    pub fn get_signature(&self) -> &str {
        &self.signature
    }

    pub fn get_user_id(&self) -> i32 {
        self.user_id
    }

    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        self.expiration < now
    }
}

fn generate_signature() -> String {
    let mut rng = rand::thread_rng();
    (0..20).map(|_| rng.sample(distributions::Alphanumeric) as char).collect()
}