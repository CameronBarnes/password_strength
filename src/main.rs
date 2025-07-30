use std::fmt::Write as _;
use std::io::{Write, stdin, stdout};
use std::path::PathBuf;

use num_format::{Locale, ToFormattedString};

fn main() {
    println!("Password Strength Checker");
    print!("Please enter your password: ");
    let _ = stdout().flush();
    let mut password = String::new();
    stdin()
        .read_line(&mut password)
        .expect("Failed to read input");
    if !password.is_ascii() {
        eprintln!("Password may only be ascii characters.");
        std::process::exit(1);
    }
    let strength = password_strength_lib::estimate_strength(
        password.trim(),
        vec![PathBuf::from("./words_alpha.txt")],
        Some(PathBuf::from("./rockyou.txt")),
    );
    print_speed_for_common_hash("MD5", 98_262_800_000., strength);
    print_speed_for_common_hash("SHA256", 13_075_800_000., strength);
    print_speed_for_common_hash("SHA512", 4_460_800_000., strength);
    // print_speed_for_common_hash("MacOS v10.4", 22_927_800_000., strength);
    print_speed_for_common_hash("NTLM", 173_300_800_000., strength);
    print_speed_for_common_hash("Samsung Android Password/PIN", 19_384_900., strength);
    print_speed_for_common_hash("1Password", 11_952_800., strength);
    print_speed_for_common_hash("bcrypt", 131_200., strength);
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn format_duration(duration: f64) -> String {
    let minutes = duration / 60.;
    let hours = minutes / 60.;
    let days = hours / 24.;
    let years = days / 365.;
    let seconds = ((minutes - minutes.floor()) * 60.) as u128;
    let minutes = ((hours - hours.floor()) * 60.) as u128;
    let hours = ((days - days.floor()) * 60.) as u128;
    let days = ((years - years.floor()) * 365.) as u128;
    let years = years as u128;

    let mut out = String::new();
    if years > 0 {
        let _ = write!(out, "{}y ", years.to_formatted_string(&Locale::en));
    }
    if days > 0 {
        let _ = write!(out, "{days}d ");
    }
    if hours > 0 {
        let _ = write!(out, "{hours}h ");
    }
    if minutes > 0 {
        let _ = write!(out, "{minutes}m ");
    }
    if seconds > 0 {
        let _ = write!(out, "{seconds}s");
    }
    out
}

fn print_speed_for_common_hash(name: &str, speed: f64, strength: f64) {
    // Statistically you're likely to run into it about half way through
    let time = (strength / speed) * 0.5;
    if time < 1. {
        println!("{name}: Instant");
    } else {
        println!("{name}: {}", format_duration(time));
    }
}
