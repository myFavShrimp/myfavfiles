use std::str::FromStr;

use bcrypt;

pub fn new_password_hash_maybe(
    password: &str,
    hash: &str,
    cost: u32,
) -> Result<Option<String>, bcrypt::BcryptError> {
    let parts = bcrypt::HashParts::from_str(hash)?;

    if cost > parts.get_cost() {
        bcrypt::hash(password, cost).map(Some)
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::new_password_hash_maybe;

    #[test]
    fn generate_new_hashwith_better_cost() {
        let password = "Xtrem Lee Secure";
        let old_hash = bcrypt::hash(password, 7).unwrap();

        let new_hash_maybe = new_password_hash_maybe(password, &old_hash, 14).unwrap();

        assert!(new_hash_maybe.is_some());
        assert!(new_hash_maybe.unwrap() != old_hash);
    }
}
