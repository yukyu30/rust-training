use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!"); //数を当ててごらん

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is:{}", secret_number); //秘密の数字は次の通り

    println!("Please input your guess."); //ほら、予想を入力してね

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Faild to read line");
    
    let guess: u32 = guess.trim().parse()
        .expect("Please type number");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
