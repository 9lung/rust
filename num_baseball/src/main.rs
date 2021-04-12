use std::io;
use rand::Rng;

fn main() {
    let mut answer = vec![0, 0, 0];

    let mut index : i32 = 0;
    while index < 3 {
        let mut is_unique = false;
        let mut generated : i32 = 0;
        while !is_unique {
            generated = rand::thread_rng().gen_range(1, 9);
            is_unique = true;
            for i in 0..index {
                if answer[i as usize] == generated {
                    is_unique = false;
                    break;
                }
            }
        }
        answer[index as usize] = generated;
        index = index + 1;
    }   

    let mut trial : i32 = 0;
    let mut strike : i32 = 0;
    while trial < 10 {
        let mut num = vec![0, 0, 0];
        
        loop { // loop for receive right input
            println!("make a new input!");
            let mut input = String::new();

            io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

            let mut out_of_bound_exception = false;
            let vec = input.split_whitespace().map(|s| s.trim());

            let mut index = 0;
            for s in vec {
                if index > 2 {
                    index = index + 1;
                    break;
                }
                num[index] = match s.parse::<i32>() {
                    Ok(i) => i,
                    Err(_e) => -1,
                  };
                if num[0] == -1 {
                    break;
                }
                index = index + 1;
            }

            if num[0] == -1 { // non number input exception
                println!("not a valid input");
                continue;
            }

            if index != 3 { // input count is not 3
                println!("should input only 3 numbers");
                continue;
            }

            for i in 0..3 {
                if num[i] < 1 || num[i] > 9 {
                    out_of_bound_exception = true;
                }
            }

            // input number is duplicated
            if num[0] == num[1] || num[1] == num[2] || num[0] == num[2] {
                println!("duplicated number not allowed");
                continue;
            }

            if out_of_bound_exception { // input number is out of bound
                println!("number out of bound or not a number");
                continue;
            }

            break;
        }

        let mut ball = 0;
        strike = 0;

        for i in 0..answer.len() {
            for j in 0..num.len() {
                if num[j] == answer[i] {
                    if i == j {
                        strike += 1;
                    } else {
                        ball += 1;
                    }
                }
            }
        }
        trial = trial + 1;
        println!("{}th trial", trial);
        println!("{} strike, {} ball", strike, ball);

        if strike == 3 {
            println!("you win!");
            break;
        }
    }

    if strike != 3 {
        println!("you lose, the answer is {}{}{}", answer[0], answer[1], answer[2]);
    }
}

