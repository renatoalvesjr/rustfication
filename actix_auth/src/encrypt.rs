pub fn hash_func(password: &str) -> String {
    use sha3::{Digest, Sha3_512};
    let mut hasher = Sha3_512::new();
    hasher.update(password);
    let result = hasher.finalize();
    format!("{:x}", result)
}
