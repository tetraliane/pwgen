use std::env::args;
use std::collections::HashSet;
use rand::{thread_rng, Rng};

fn print_help() {
    println!("pwgen FAMILIES [LENGTH]");
    println!("Generates a random password. FAMILIES must be a string of these characters.");
    println!("  a: lower case alphabets");
    println!("  A: upper case alphabets");
    println!("  n: numbers");
    println!("  s: symbols");
}

fn chars(family: char) -> Vec<char> {
    (match family {
        'a' => "abcdefghijklmnopqrstuvwxyz",
        'A' => "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
        'n' => "0123456789",
        's' => "!?#$%&@_,.+-*/^=~\"'()[]{}<>",
        _ => {
            eprintln!("pwgen: Unknown character family: '{}'", family);
            ""
        }
    }).chars().collect()
}

fn select_chars(families: HashSet<char>) -> Vec<char> {
    families.into_iter().map(|fam| chars(fam)).flatten().collect()
}

fn pickup(characters: &[char]) -> char {
    let mut rng = thread_rng();
    let ind: usize = rng.gen_range(0..characters.len());
    characters[ind]
}

fn pwgen(characters: &[char], len: u8) -> String {
    (0..len).map(|_| pickup(characters)).collect()
}

fn main() {
    let mut args = args();
    args.next();
    match args.next().as_deref() {
        Some("help") | Some("--help") | Some("-h") | None => print_help(),
        Some(x) => {
            let families: HashSet<char> = x.chars().collect();
            let len: u8 = match args.next() {
                Some(y) => y.parse().unwrap(),
                None => 15,
            };
            println!("{}", pwgen(&select_chars(families), len))
        },
    }
}
