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
