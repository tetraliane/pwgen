use std::{env::args, process};

mod pwgen;

use pwgen::Family;

const HELP_MESSAGE: &'static str = concat!(
    "Usage: pwgen FAMILIES [LENGTH]\n",
    "Generates a random password. FAMILIES must consist of these characters.\n",
    "  a: lower case alphabets\n",
    "  A: upper case alphabets\n",
    "  n: numbers\n",
    "  s: symbols\n",
);

fn parse_families(families: &[char]) -> Result<Vec<Family>, FamilyError> {
    families
        .into_iter()
        .map(|f| match f {
            'a' => Ok(Family::LowerAlph),
            'A' => Ok(Family::UpperAlph),
            'n' => Ok(Family::Numbers),
            's' => Ok(Family::Symbols),
            _ => Err(FamilyError { actual: *f }),
        })
        .collect()
}

#[derive(Debug)]
struct FamilyError {
    actual: char,
}

impl std::fmt::Display for FamilyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid family: \"{}\"", self.actual)
    }
}

impl std::error::Error for FamilyError {}

fn main() {
    let mut args = args();
    args.next();
    match args.next().as_deref() {
        Some("help") | Some("--help") | Some("-h") | None => print!("{}", HELP_MESSAGE),
        Some(x) => {
            let families = parse_families(&x.chars().collect::<Vec<_>>()).unwrap_or_else(|e| {
                eprintln!("pwgen: {}", e);
                process::exit(1)
            });
            let len: u8 = match args.next() {
                Some(y) => y.parse().unwrap(),
                None => 15,
            };
            println!("{}", pwgen::pwgen(&families, len, &mut rand::thread_rng()))
        }
    }
}
