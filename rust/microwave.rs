use std::{env, thread, time};
use std::io::{self, Write};


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("No cook time provided.");
    }
    let frequency = 10;
    let sleep_time = time::Duration::from_secs_f64(1.0 / frequency as f64);
    let cook_time_secs = match args[1].parse::<usize>() {
        Ok(usize) => usize,
        Err(_) => panic!("Invalid or missing cook time provided."),
    };
    let cook_time_mins_display = cook_time_secs / 60;
    let cook_time_secs_display = cook_time_secs % 60;
    println!("COOK TIME: {:02}:{:02}", cook_time_mins_display, cook_time_secs_display);
    let reps = cook_time_secs * frequency;
    println!("BEEP!");
    for _ in 0..reps {
        print!("m");
        io::stdout().flush().unwrap();
        thread::sleep(sleep_time);
    }
    println!("\nBEEEP! BEEEP! BEEEP!");
    println!("Food's done! :)");
}
