use cryptal::rng::Csprng;
use std::sync::{Mutex, OnceLock};

static RNG: OnceLock<Mutex<Csprng>> = OnceLock::new();

fn get_rng() -> &'static Mutex<Csprng> {
    RNG.get_or_init(|| Mutex::new(Csprng::new()))
}

pub fn random_fill(buffer: &mut [u8]) {
    let mut rng = get_rng().lock().unwrap();

    rng.fill_bytes(buffer);
}
