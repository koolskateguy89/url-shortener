use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

fn salt() -> SaltString {
    SaltString::generate(&mut OsRng)
}

/// Argon2 with default params (Argon2id v19)
fn argon2<'a>() -> Argon2<'a> {
    Argon2::default()
}

// Hide implementation details of password hashing by returning Option
pub fn hash_password(password: &[u8]) -> Option<String> {
    let salt = salt();
    let argon2 = argon2();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2.hash_password(password, &salt).ok()?.to_string();

    Some(password_hash)
}

/// Verify password against PHC string.
pub fn verify_password(password: &[u8], password_hash: &str) -> bool {
    // NOTE: hash params from `parsed_hash` are used instead of what is configured in the
    // `Argon2` instance.
    let parsed_hash = if let Ok(parsed_hash) = PasswordHash::new(password_hash) {
        parsed_hash
    } else {
        return false;
    };

    argon2().verify_password(password, &parsed_hash).is_ok()
}

pub fn validate_credentials(username: &str, password: &str) -> bool {
    // ensure username is only a-zA-Z0-9
    if !username.chars().all(|c| c.is_ascii_alphanumeric()) {
        return false;
    }

    if password.len() < 4 {
        return false;
    }

    if username.len() < 3 {
        return false;
    }

    true
}
