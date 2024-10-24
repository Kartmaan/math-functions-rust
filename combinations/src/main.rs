/// Returns the factorial of a number n
fn fact(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        let mut res: u64 = 1;
        for i in 1..= n {
            res *= i;
        }
        res
    }
}

/// # binomial_combi
/// Returns the binomial coefficient, which represents the 
/// number of ways to choose k elements from n elements, without 
/// repetition.
/// ## Example
/// Choosing 2 letters from {A, B, C} without repetition gives 
/// 3 possibilities: AB, AC, BC. 
/// This is equivalent to n=3, k=2
fn binomial_combi(n:u64, k:u64) -> u64 {
    if n > 0 && k > 0 {
        let res: u64 = fact(n) / (fact(k) * fact(n-k));
        res
    } else {
        0
    }
}

/// # multiset_combi
/// Returns the number of multi-set combinations of k elements 
/// among n elements, which represents the number of ways to 
/// choose k elements among n elements with possible repetition.
/// ## Example
/// Choosing 2 letters from {A, B, C} with repetition gives 
/// 6 possibilities: AA, AB, AC, BB, BC, CC.
/// This is equivalent to n=3, k=2
fn multiset_combi(n:u64, k:u64) -> u64 {
    if n > 0 && k > 0 {
        let res: u64 = fact(n + k - 1) / (fact(k) * (n-1));
        res
    } else {
        0
    }
}

fn main() {
    let n:u64 = 3;
    let k:u64 = 2;

    println!("Binomial ({},{}) : {}", n, k, binomial_combi(n, k));
    println!("Multiset ({},{}) : {}", n, k, multiset_combi(n, k));
}