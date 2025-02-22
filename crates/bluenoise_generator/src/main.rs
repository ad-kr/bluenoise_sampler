use bluenoise::WrappingBlueNoise;
use rand::rngs::SmallRng;

const SIZE: f32 = 256.0;
const RADIUS: f32 = 8.0;

fn main() {
    let noise = WrappingBlueNoise::<SmallRng>::from_seed(SIZE, SIZE, RADIUS, 972305808234);
    let mut values = noise.map(|vec| (vec.x, vec.y)).collect::<Vec<_>>();

    values.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    println!("Generated {} points", values.len());
    println!("{values:?}");
}
