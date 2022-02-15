use std::io;
use rand::Rng;


fn main() {
    println!("Number guessing Game");

    println!("Your guess:");

    let sn:u32 = rand::thread_rng().gen_range(1..101);

        loop{
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("you guessed: {}" , guess);


        if guess > sn {
            println!("The number is less than {}", guess);
        }
        else if guess < sn {
            println!("The number is greater than {}", guess);
        }
        else{
            println!("The number is {}!", guess);
            break;
        }
    }

}
