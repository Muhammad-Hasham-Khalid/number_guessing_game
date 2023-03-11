use rand::{seq::SliceRandom, thread_rng};
use std::io;

fn generate_random_int() -> i32 {
    let mut rng = thread_rng();

    let mut nums: Vec<i32> = (1..100).collect();
    nums.shuffle(&mut rng);
    return nums[0];
}

fn get_user_input() -> i32 {
    let mut input_buff = String::new();

    println!("Enter your guess: ");
    match io::stdin().read_line(&mut input_buff) {
        Ok(_n) => {}
        Err(error) => println!("error in reading user input: {}", error),
    }
    let mut input: i32 = 0;
    match input_buff.trim().parse::<i32>() {
        Ok(i) => input = i,
        Err(err) => println!("An error occured while parsing: {}", err),
    };
    return input;
}

fn main() {
    let generated = generate_random_int();
    let mut user_input = get_user_input();
    let mut tries = 1;

    while user_input != generated {
        if tries >= 10 {
            println!("Sorry! you ran out of tries, the number was {}", generated);
            break;
        }
        if user_input > generated - 5 && user_input < generated + 5 {
            println!("You are very close!");
        } else if user_input < generated {
            println!("Go a bit higher!");
        } else if user_input > generated {
            println!("Go a bit lower!");
        }

        user_input = get_user_input();
        tries += 1;
    }

    if user_input == generated {
        println!("Congratulationsss!!");
    }
}
