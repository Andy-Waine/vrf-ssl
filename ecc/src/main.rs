use rand::{thread_rng, Rng};

fn main() {
    let p: f64 = 694857311.0;        // this dummy value will be replaced by the VRF call (any prime integer)
    let a: f64 = 26712813.0;        // this dummy value will be replaced by the VRF call (any integer less than p)
    let b: f64 = 47964189.0;        // this dummy value will be replaced by the VRF call (any integer less than p)
    let bob_priv_key: f64 = 319118189;   // this dummy value will be replaced by the VRF call (any integer less than p)
    let alice_priv_key: f64 = 289125298; // this dummy value will be replaced by the VRF call (any integer less than p)

    let alice_y = calculate_y(alice_priv_key, a, b, p);
    let bob_y = calculate_y(bob_priv_key, a, b, p);
    println!("alice public key: {}", alice_y);
    println!("bob public key: {}", bob_y);

    let alice_shared_key = (bob_y * alice_priv_key) % p;
    let bob_shared_key = (alice_y * bob_priv_key) % p;
    println!("alice shared key: {}", alice_shared_key);
    println!("bob shared key: {}", bob_shared_key);

}

/*
*   @notice y^2 = x^3 + ax + b (mod p)
*   @param x: random int generated (private)
*   @param a: random int generated (public)
*   @param b: random int generated (public)
**/
fn calculate_y(x: f64, a: f64, b: f64, p: f64) -> f64 {
    let y_squared = (x.powf(3.0) + a*x + b);
    let y = (y_squared.sqrt() % p);
    return y;
}

// fn ecc_dh_key_exchange(alice_priv_key: u128, bob_priv_key: u128, a: u128, b: u128, p: u128) -> u128 {
//     // Calculate Alice's public key
//     let alice_pub_key = (alice_priv_key.pow(3) + (a * alice_priv_key) + b) % p;
//     println!("Alice's public key is: {}", alice_pub_key);

//     // Calculate Bob's public key
//     let bob_pub_key = (bob_priv_key.pow(3) + (a * bob_priv_key) + b) % p;
//     println!("bob's public key is: {}", bob_pub_key);

//     // Calculate Alice's shared key
//     let alice_shared_key = (bob_pub_key * alice_priv_key) % p;

//     // Calculate Bob's shared key
//     let bob_shared_key = (alice_pub_key * bob_priv_key) % p;

//     println!("Alice's shared key is: {}", alice_shared_key);
//     println!("Bob's shared key is: {}", bob_shared_key);

//     // Return the shared key
//     (alice_shared_key == bob_shared_key) as u128
// }
