use rand::Rng;

/// Tests a probability by returning a bool
fn proba_bool(proba: f32) -> bool {
    if proba > 0.0 && proba < 100.0 {
        let proba_coef: f32 = proba / 100.0;
        let rng_num: f32 = rand::thread_rng().gen();
        if rng_num < proba_coef {
            true
        } else {
            false
        }
    } else {
        println!("Incorrect proba value (>0, <100)");
        false
    }
}

/// Tests a probability by returning an Option<bool>
fn proba_some(proba: f32) -> Option<bool> {
    if proba > 0.0 && proba < 100.0 {
        let proba_coef: f32 = proba / 100.0;
        let rng_num: f32 = rand::thread_rng().gen();
        if rng_num < proba_coef {
            Some(true)
        } else {
            Some(false)
        }
    } else {
        None
    }
}

/// Tests a probability by returning a Result<bool, String>
fn proba_result(proba:f32) -> Result<bool, String>{
    if proba > 0.0 && proba < 100.0 {
        let proba_coef: f32 = proba / 100.0;
        let rng_num: f32 = rand::thread_rng().gen();

        if rng_num < proba_coef {
            Ok(true)
        } else {
            Ok(false)
        }

    } else {
        return Err(String::from("Incorrect proba value (>0, <100)"));
    }
}

/// The function tests a chosen probability, several attempts 
/// will be made until the probability is realized. If an attempt 
/// fails the function displays 'x', otherwise the loop stops and 
/// displays 'win' followed by some statistics.
/// The function takes as parameter a probability value between 
/// 0 and 100 ('proba')
fn proba_attemps(proba:f32) {
    if proba <= 0.0 || proba > 100.0 {
        println!("Incorrect proba value (>0, <100)");
    } else {
        let proba_coef: f32 = proba / 100.0;
        let mut i: i32 = 0; // Attempts counter
        
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
    // Probability value (1..100)
    let probability: f32 = 3.33;

    // Using 'proba_bool' with 'if..else' statement
    if proba_bool(probability) {
        println!("Probability realized");
    } else {
        println!("Probability not realized");
    }

    // Using 'proba_some' with 'if let' synthax
    let lets_try: Option<bool> = proba_some(probability);
    if let Some(res) = lets_try {
        if res { // true
            println!("Probability realized");
        } else { // false
            println!("Probability not realized");
        }
    } else { // None (Error)
        println!("Incorrect value");
    }

    // Using 'proba_result'
    match proba_result(probability) {
        Ok(win) => {
            if win {
                println!("Probability realized");
            } else {
                println!("Probability not realized");
            }
        }
        Err(error) => {
            println!("Error : {}", error);
        }
    }

    // Using 'proba_attemps'
    proba_attemps(probability);
}