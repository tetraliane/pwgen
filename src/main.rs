use std::collections::HashSet;
use std::env::args;

use rand::{thread_rng, Rng};

const HELP_MESSAGE: &'static str = concat!(
    "Usage: pwgen FAMILIES [LENGTH]\n",
    "Generates a random password. FAMILIES must consist of these characters.\n",
    "  a: lower case alphabets\n",
    "  A: upper case alphabets\n",
    "  n: numbers\n",
    "  s: symbols\n",
);

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
    })
    .chars()
    .collect()
}

fn select_chars(families: HashSet<char>) -> Vec<char> {
    families.into_iter().map(chars).flatten().collect()
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
        Some("help") | Some("--help") | Some("-h") | None => print!("{}", HELP_MESSAGE),
        Some(x) => {
            let families: HashSet<char> = x.chars().collect();
            let len: u8 = match args.next() {
                Some(y) => y.parse().unwrap(),
                None => 15,
            };
            println!("{}", pwgen(&select_chars(families), len))
        }
    }
}
