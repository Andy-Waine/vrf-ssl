use rand::{thread_rng, Rng};

fn main() {
    let rand_p = 694857286.0        // this dummy value will be replaced by the VRF call (any prime integer)
    let rand_a = 26712813.0;        // this dummy value will be replaced by the VRF call (any integer less than p)
    let rand_b = 47932189.0;        // this dummy value will be replaced by the VRF call (any integer less than p)
    let bob_priv_key = 919118189;   // this dummy value will be replaced by the VRF call (any integer less than p)
    let alice_priv_key = 289125298; // this dummy value will be replaced by the VRF call (any integer less than p)

    let resultant_y = calculate_y(rand_x, rand_a, rand_b);
    println!("resultant_y: {}", resultant_y);
    let resultant_y_sqaured = resultant_y.powf(2.0);
    println!("resultant_y^2: {}", resultant_y_sqaured);
    let x_cubed = rand_x.powf(3.0);
    println!("x^3: {}", x_cubed);
    let ax = rand_a * rand_x;
    println!("a*x: {}", ax);
    let is_y_squared_different = x_cubed + ax + rand_b;
    println!("is_y_squared_different: {}", is_y_squared_different); 
    let is_y_different = is_y_squared_different.sqrt();
    println!("is_y_different: {}", is_y_different); 
}

/*
*   @notice y^2 = x^3 + ax + b
*   @param x_private_key: random int generated (private)
*   @param a: random int generated (public)
*   @param b: random int generated (public)
**/
fn calculate_y(x_private_key: f64, a: f64, b: f64) -> f64 {
    let y_squared = (x.powf(3.0) + a*x + b);
    let y = y_squared.sqrt();
    return y;
}
