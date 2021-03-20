extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1,101); // 乱数を生成

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess) // Inputを受け取る
            .expect("Fail to read line!");
        
        let guess: u32 = match guess.trim().parse() { // trimで前後空白を削除して、parseで数値化
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) { // guessとsecret_numberを比較する
            Ordering::Less => println!("small"),
            Ordering::Greater => println!("large"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        println!("You guessed : {}", guess);
    }
}
