use bcrypt::{hash, verify, DEFAULT_COST};

pub fn password_hashing(password: &str) -> String {
    hash(password, DEFAULT_COST).expect("Failed to hash password")
}


pub fn verify_password(password: &str, hashed_password: &str) -> bool {
    verify(password, hashed_password).expect("Failed to compare password hash with password!")
}
