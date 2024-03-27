use std::time::SystemTime;

use rand::{distributions, Rng};

pub const AUTH_TOKEN: &str = "auth-token";

#[derive(Debug, PartialEq)]
pub struct AuthToken {
    user_id: i32,
    expiration: u64, // UNIX EPOCH TIME STAMP as u64
    signature: String,
}

impl AuthToken {
    pub fn new(user_id: i32) -> Self {
        const COOKIE_EXPIRATION_TIME_IN_SECONDS: u64 = 60 * 60; // 1 hour
        let now_in_secs = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let expiration = now_in_secs + COOKIE_EXPIRATION_TIME_IN_SECONDS;

        let signature = generate_signature();

        AuthToken {
            user_id,
            expiration,
            signature,
        }
    }

    pub fn from_str(token: &str) -> Option<Self> {
        let mut iter = token.split_inclusive("-");
        if iter.next() != Some("user-") {
            return None;
        }
        let user_id = match iter.next() {
            Some(id) => match remove_last_char(id).parse() {
                Ok(num) => num,
                Err(_) => return None,
            },
            None => return None,
        };
        let expiration = match iter.next() {
            Some(expiration) => {
                let parsed = remove_last_char(expiration).parse::<u64>();
                match parsed {
                    Ok(a) => a,
                    Err(_) => {
                        return None;
                    }
                }
            }
            None => return None,
        };
        let signature = iter.collect::<Vec<&str>>().join("");

        Some(AuthToken {
            user_id,
            expiration,
            signature,
        })
    }
}

impl AuthToken {
    pub fn to_str(&self) -> String {
        format!(
            "user-{}-{}-{}",
            self.user_id, self.expiration, self.signature
        )
    }

    pub fn get_signature(&self) -> &str {
        &self.signature
    }

    pub fn get_user_id(&self) -> i32 {
        self.user_id
    }

    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.expiration < now
    }
}

fn remove_last_char(string: &str) -> &str {
    let mut chars = string.chars();
    chars.next_back();
    chars.as_str()
}

#[test]
fn rem_last_char_test() {
    assert_eq!(remove_last_char("abc"), "ab");
    assert_eq!(remove_last_char(""), "");
    assert_eq!(remove_last_char("abä"), "ab");
}

fn generate_signature() -> String {
    let mut rng = rand::thread_rng();
    (0..20)
        .map(|_| rng.sample(distributions::Alphanumeric) as char)
        .collect()
}

#[test]
fn sig_length() {
    let sig = generate_signature();

    assert_eq!(sig.len(), 20)
}

#[test]
fn normal_token_with_minus_in() {
    let token = AuthToken::from_str("user-1-12-a-b").unwrap();

    assert_eq!(token.signature, "a-b");
    assert_eq!(token.user_id, 1);
    assert_eq!(token.expiration, 12);
}

#[test]
fn none_tokens_from_str() {
    assert_eq!(AuthToken::from_str(""), None);
    assert_eq!(AuthToken::from_str("user"), None);
    assert_eq!(AuthToken::from_str("user-1"), None);
    assert_eq!(AuthToken::from_str("user-1-1"), None);
}
