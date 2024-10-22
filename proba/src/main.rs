use rand::Rng;

fn attemps(proba:u8) {
    if proba <= 0 || proba > 100 {
        println!("Incorect proba value");
    } else {
        let proba_coef: f32 = proba as f32 / 100.0;
        let mut i: i32 = 0;
        
        loop {
            let rng_num: f32 = rand::thread_rng().gen();

            if rng_num <= proba_coef {
                i += 1;
                println!("WIN!");
                println!("Probability : {}% | Won in {} attemps", proba, i);
                println!("Target value : {} | Value obtained {}", proba_coef, rng_num);
                break;

            } else {
                print!("x.");
                i += 1;
            }
        } // loop
    } // else (good proba value)
} // fn attemps

fn main() {
    let probability: u8 = 1;
    attemps(probability);
}
