pub mod auth;

/// Returns a random 6 character string.
pub fn random_url_id() -> String {
    nanoid::nanoid!(6)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_url_id() {
        let id = random_url_id();
        assert_eq!(id.len(), 6);

        let another_id = random_url_id();
        assert_ne!(id, another_id);
    }
}
