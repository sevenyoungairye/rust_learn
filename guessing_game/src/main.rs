use std::io;
use rand::Rng;
use std::cmp::Ordering;

/**
 * demo address: 
 * https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
 */
fn main() {

    println!("guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    // while loop.
    loop {
        println!("pls input your guess number.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
    
        // trim blank.
        // let guess :u32 = guess.trim()
        //     .parse()
        //     .expect("哒咩, 类型转换错误, 请输入数字类型。");

        // 遇到转换错误，退出当前循环
        let guess :u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guessed: {guess}");
        
        // match exp.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small."),
            Ordering::Greater => println!("too big."),
            Ordering::Equal => {
                println!("u win!");
                break;
            },
        }
    }

    // if guess.contains("cppd") {
    //     // todo: contains is invalid.
    //     println!("cpdd, u'r only.")
    // }
}