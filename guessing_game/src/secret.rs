use rand::Rng;

pub fn get_secret () -> u32 {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    secret_number
}