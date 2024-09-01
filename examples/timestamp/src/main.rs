use clap::Parser;
use chrono::{DateTime, Duration, Local, Utc, TimeDelta};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// date format
    #[arg(short, long)]
    format: String,

    /// Number of seconds
    #[arg(short, long, default_value_t = 0)]
    seconds: u8,
}

fn main() {
    let args = Args::parse();

    println!("format - {}, seconds: {}", args.format, args.seconds);
    let local_time = Local::now();
    println!("Local time now is {}", local_time);
    println!("Local time now is {}", local_time.format(&args.format));
    let duration = TimeDelta::seconds(args.seconds.into());
    let nextTs = local_time + duration;
    println!("After {} seconds {}", args.seconds, nextTs.format(&args.format));
}
