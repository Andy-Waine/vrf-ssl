use rand::{thread_rng, Rng};

fn main() {
    let bob_priv_key = generate_private_key();
    let alice_priv_key = generate_private_key();
    let rand_x = 78765431.0;     // this dummy value will be replaced by the VRF call
    let rand_a = 26712813.0;     // this dummy value will be replaced by the VRF call
    let rand_b = 47932189.0;     // this dummy value will be replaced by the VRF call
    let resultant_y = calculate_y(rand_x, rand_a, rand_b);
    println!("resultant_y: {}", resultant_y);
    println!("resultant_y^2: {}", resultant_y.powf(2.0));
    println!("x^3: {}", rand_x.powf(3.0));
    let ax = rand_a * rand_x;
    println!("a*x: {}", ax);
}

/*
*   @notice Uses the NIST formula  (y^2 = x^3 + ax + b)
*   @param x: random number generated (private)
*   @param a: random number generated (public)
*   @param b: random number generated (public)
**/
fn calculate_y(x: f64, a: f64, b: f64) -> f64 {
    let y_squared = (x.powf(3.0) + a*x + b);
    let y = y_squared.sqrt();
    return y;
}

fn generate_private_key() -> f64 {
    let mut rng = thread_rng();
    let priv_key: f64 = rng.gen();
    return priv_key;
}
