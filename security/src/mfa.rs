use rand::Rng;

pub fn enroll_user(user_id: &str) -> bool {
    let _ = user_id;
    true
}

pub fn verify_user(user_id: &str) -> bool {
    let _ = user_id;
    let mut rng = rand::thread_rng();
    rng.gen_bool(0.9)
}
