use rand::{thread_rng, Rng};

fn pickup(characters: &[String]) -> String {
    let mut rng = thread_rng();
    let ind: usize = rng.gen_range(0..characters.len());
    characters[ind].clone()
}

fn pwgen(characters: &[String], len: u8) -> String {
    if len == 0 {
        String::new()
    } else {
        pwgen(characters, len-1) + &pickup(characters)
    }
}

fn main() {
    let ab = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
        'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];
    let ab_lower: Vec<String> = ab.map(|x| x.to_string()).into();
    let ab_upper: Vec<String> = ab.map(|x| x.to_string().to_uppercase()).into();
    let numbers = Vec::from_iter((0..10).map(|x: u8| x.to_string()));
    let characters = [ab_lower, ab_upper, numbers].concat();

    let pw = pwgen(&characters, 15);
    println!("{}", pw);
}
