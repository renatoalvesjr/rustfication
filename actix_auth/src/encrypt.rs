use argon2::{password_hash::*, Argon2};
use rand_core::OsRng;

pub fn password_hash(password: &str) -> String {
    let salt: SaltString = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash: String = argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string();

    //parsed hash to store on the db
    //$argon2id$v=<version>m=<memory_cost>t=<iterations>p=<parallelism>$<salt>$<hash>
    let parsed_hash: PasswordHash<'_> =
        PasswordHash::new(&password_hash).expect("Failed to parse password hash");
    return parsed_hash.to_string();
}

pub fn password_verify(password: &str, hash: &str) -> bool {
    let argon2 = Argon2::default();
    let parsed_hash: PasswordHash<'_> =
        PasswordHash::new(hash).expect("Failed to parse password hash");

    argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}
