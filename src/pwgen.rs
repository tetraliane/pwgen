use rand::Rng;

pub fn pwgen<R: Rng>(families: &[Family], len: u8, rng: &mut R) -> String {
    let characters = families.iter().flat_map(Family::chars).collect::<Vec<_>>();
    (0..len).map(|_| pickup(&characters, rng)).collect()
}

fn pickup<R: Rng>(characters: &[char], rng: &mut R) -> char {
    let ind: usize = rng.gen_range(0..characters.len());
    characters[ind]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Family {
    LowerAlph,
    UpperAlph,
    Numbers,
    Symbols,
}

impl Family {
    pub(crate) fn chars(&self) -> Vec<char> {
        match self {
            Self::LowerAlph => "abcdefghijklmnopqrstuvwxyz".chars().collect(),
            Self::UpperAlph => "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
            Self::Numbers => "0123456789".chars().collect(),
            Self::Symbols => "!?#$%&@_,.+-*/^=~\"'()[]{}<>".chars().collect(),
        }
    }
}
