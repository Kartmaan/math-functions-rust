/// Determines if a number is prime
fn is_prime(n:u64) -> bool {
    if n <= 1 {
        return false; // Not prime
    }

    // a number 'n' is prime if it's not divisible by any number 
    // smaller than its square root
    for i in 2..=(n as f64).sqrt() as u64 {
        if n % i == 0 {
            return false; // Not prime
        }
    }
    true // Prime
}

fn main() {
    for num in 1..2000 {
        if is_prime(num as u64) {
            print!("{}", num);
        } else {
            print!(".");
        }
    }
}