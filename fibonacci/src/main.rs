/// Generates the first k Fibonacci numbers in a vector.
/// 
/// Since the Fibonacci sequence grows (almost) exponentially, 
/// the returned vector contains u64 values type. So the maximum 
/// value this vector can contain is (2^64)-1
fn fibo(k:i32) -> Vec<u64> {
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
            fibo_vec.push(fibo_num);
            i += 1;
        }
        fibo_vec

    } else {
        println!("ERROR");
        vec![0]
    }
}

fn main() {
    let fibo_vals: Vec<u64> = fibo(20);
    println!("{:?}", fibo_vals);
}