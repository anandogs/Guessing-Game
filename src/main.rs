use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut count: u32 = 0;

    loop {
        println!("Please input a number");
        
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => { 
                println!("Too Small!");
                count = count + 1;
            }
            Ordering::Equal => { 
                println!("You Win!");
                println!("Your score is {}. The lower the better!", count);
                break;
            }
            Ordering::Greater => {
                println!("Too Big!");
                count = count + 1;
            }
        }
    }
        

}
