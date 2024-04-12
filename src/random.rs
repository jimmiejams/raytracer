use rand::random;

pub fn random_range(min: f32, max: f32) -> f32 {
    min + (max - min) * rand::random::<f32>()
}