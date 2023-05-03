fn main() {
    let modulus = 34022366920938;  // random integer obtained by vrf
    print_num_digits(modulus);
    find_next_prime(modulus);
}

fn find_next_prime(num_generated: u128) -> u128 {
    let mut n = num_generated + 1;
    while !is_prime(n) {
        println!("N tested for primality: {}", n);
        n += 1;
    }
    println!("This N is Prime: {}", n);
    n
}
 
fn is_prime(num: u128) -> bool {
    if num == 2 || num == 3 {
        return true;
    }
    if num <= 1 || num % 2 == 0 || num % 3 == 0 {
        return false;
    }
    for i in (5..=((num as f64).sqrt() as u128)).step_by(6) {
        if num % i == 0 || num % (i + 2) == 0 {
            return false;
        }
    }
    true
}

fn print_num_digits(num: u128) {
    let mut n = num;
    let mut count = 0;
    while n > 0 {
        n /= 10;
        count += 1;
    }
    println!("Number of digits in input is {}", count);
}
