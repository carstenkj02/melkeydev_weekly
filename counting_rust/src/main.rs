use std::io::prelude::*;                                                           

fn main() -> Result<(), ()> {
    answer("Do you wanna start counting")?;
    let amount = read_num("How much do you want to count to? ");
    println!("Press to enter to count!");
    loop {
        count(amount)?;
    }
}

fn count(num: usize) -> Result<(), ()> {
    let mut tmp = String::new();
    for i in 1..num+1 {
        print!("{}", i);
        std::io::stdout().flush().ok().expect("Could not flush stdout");
        if i != num {
            std::io::stdin().read_line(&mut tmp).expect("Invalid UTF-8 characters found in input");
        }
    }
    println!("");
    answer("Do you want to keep going")
}

fn read_num(prompt: &str) -> usize {
    print!("{}", prompt);
    loop {
        std::io::stdout().flush().ok().expect("Could not flush stdout");
        let mut input_str = String::new();
        std::io::stdin().read_line(&mut input_str).expect("Invalid UTF-8 characters found in input");
        match input_str.trim().parse::<usize>() {
            Ok(num) => return num,
            Err(_) => println!("Not a number, please try again."),
        }
    }
}

fn answer(prompt: &str) -> Result<(), ()> {
    print!("{} [Y/n]: ", prompt);
    std::io::stdout().flush().ok().expect("Could not flush stdout");
    let mut input_str = String::new();
    std::io::stdin().read_line(&mut input_str).expect("Invalid UTF-8 characters found in input");
    match input_str.trim() {
        "y" | "Y" => Ok(()),
        _ => {
            println!("Cancelled by user");
            Err(())
        },
    }
}
