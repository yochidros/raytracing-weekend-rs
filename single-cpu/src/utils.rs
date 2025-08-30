use std::sync::Mutex;

use once_cell::sync::Lazy;
use rand::{distr::Uniform, rngs::StdRng, Rng, SeedableRng};

// static な分布と生成器を 1 回だけ作って、毎回それを使用
static RNG: Lazy<Mutex<StdRng>> = Lazy::new(|| Mutex::new(StdRng::seed_from_u64(5464)));
static DIST: Lazy<Uniform<f32>> = Lazy::new(|| Uniform::new(0.0, 1.0).unwrap());

#[inline]
pub fn f32_random() -> f32 {
    let mut rng = RNG.lock().unwrap();
    rng.sample(*DIST)
}

#[inline]
pub fn f32_random_range(min: f32, max: f32) -> f32 {
    min + (max - min) * f32_random()
}

