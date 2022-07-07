use std::env::args;
use std::collections::HashSet;
use rand::{thread_rng, Rng};

fn chars(family: char) -> Vec<char> {
    (match family {
        'a' => "abcdefghijklmnopqrstuvwxyz",
        'A' => "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
        'n' => "0123456789",
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
    let args: Vec<String> = args().collect();
    let families: HashSet<char> = args[1].chars().collect();
    let len: u8 = if args.len() > 2 { args[2].parse().unwrap() } else { 15 };
    println!("{}", pwgen(&select_chars(families), len));
}
