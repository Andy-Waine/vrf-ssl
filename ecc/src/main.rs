use rand::{thread_rng, Rng};

fn main() {
    let alice_priv_key = generate_private_keys();
    let bob_priv_key = generate_private_keys();
    generate_elliptic_curve();
}

/*
*   x: bob's private key
*   a: alice's private key
*   b: any real number
**/
fn generate_elliptic_curve(x: i32, a: i32, b: i32) -> i32 {
    let y_squared = x.pow(3) + a*x + b;
    return y_squared;
}

fn generate_private_key() -> i32 {
    let mut rng = thread_rng();
    let x: i32 = rng.gen();
    return x;
}
