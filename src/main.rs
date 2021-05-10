use std::{
    io::{stdout, Write},
    time::Duration,
    thread::sleep,
};
extern crate clap;
use clap::{Arg, App};


fn main() {
    let matches = App::new("Pomd")
        .version("0.0")
        .author("pomd")
        .about("pom timer")
        .arg(Arg::with_name("DURATION")
            .short("d")
            .long("duration")
            .value_name("25")
            .help("Sets the duration the timer runs for. duration=minutes")
            .takes_value(true))
        .get_matches();

    let interval = Duration::from_millis(1000);
    let duration = matches.value_of("DURATION").unwrap_or("25").parse::<i32>().unwrap();
    let mut time_remaining = duration * 60;
    let mut stdout = stdout();

    println!("Pomd");
    println!("--------");
    while time_remaining > 0 {
        let mins = time_remaining / 60;
        let remaining_secs = time_remaining - (mins * 60);
        let secs = remaining_secs;

        print!("\r🍅 Time left: {:0>2}:{:0>2} mins", mins.to_string(), secs.to_string());
        stdout.flush().unwrap();
        sleep(interval);
        time_remaining = time_remaining-1;
    }
}
