use rand::prelude::*;
use rand_distr::Normal;

fn main() {
    let rng = StdRng::seed_from_u64(1);
    histagram::histagram(rng.sample_iter(Normal::new(0.0, 1.0).unwrap()).take(1000));
}
