/// Generates the first k Fibonacci numbers in a vector.
fn fibo_gen(k:i32) -> Vec<u64> {
    let mut fibo_vec: Vec<u64> = vec![0, 1]; // Initial vector

    // For small values of k, the initial vector is returned, 
    // or a truncated version if k is equal to 1.
    if k > 0 && k < 3 {
        match k {
            1 => fibo_vec[..1].to_vec(),
            2 => fibo_vec,
            _ => fibo_vec,
        }

    // Iteration process generating Fibonacci numbers as long as 
    // vector length is less than k 
    } else if k >= 3 {
        let mut i: usize = 0;
        while fibo_vec.len() < k as usize {
            let fibo_num: u64 = fibo_vec[i] + fibo_vec[i+1];
            fibo_vec.push(fibo_num); // Adding a new number
            i += 1;
        }
        fibo_vec

    } else {
        println!("ERROR");
        vec![0]
    }
}

/// Checks if a number is a Fibonacci number
fn is_fibo(n: u64) -> bool {
    let mut i: i32 = 1;

    // No need to iterate
    if n == 0 {
        return true;

    // Iteration process
    // Each time a loop is passed, a vector containing 'i' 
    // Fibonacci numbers is created, and if it contains the 
    // value 'n', the function returns 'true'. Otherwise, a new 
    // vector is created with an additional Fibonacci number 
    // to attempt a new check. If the last value of the vector 
    // is greater than 'n', then 'n' is not a Fibonacci number 
    // (the function returns 'false').
    } else {
        loop {
            let fibo_vec: Vec<u64> = fibo_gen(i);
            if fibo_vec.contains(&n) {
                return true;
            } else {
                if fibo_vec[fibo_vec.len() - 1] > n {
                    return false;
                } else {
                    i += 1;
                    continue;
                }
            }
        }// loop
    }// else n > 1
}

fn main() {
    let fibo_vals: Vec<u64> = fibo_gen(20);
    let num1: u64 = 144;
    let num2: u64 = 145;

    println!("{:?}", fibo_vals);
    println!("Is {} a Fibonacci number : {:?}", num1, is_fibo(num1));
    println!("Is {} a Fibonacci number : {:?}", num2, is_fibo(num2));
}