use clap::error::ContextValue::Bool;
use rand::Rng;
use rand_seeder::Seeder;
use rand_pcg::Pcg64;

pub fn generate_random(seed: String, width: u32) -> Vec<bool> {
    // make the output vector
    let mut out: Vec<bool> = Vec::new();

    // make the seeded rng
    let mut rng: Pcg64 = Seeder::from(seed).make_rng();

    // populate the output vector
    for _ in 0..width {
        out.push(rng.gen::<bool>());
    }

    return out;
}