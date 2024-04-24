use std::{collections::HashSet, ops::Index};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Alphabet {
    chars: Vec<char>, // store as vec to conserve order
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Error {
    DuplicateChar(char),
    NotSubset(char),
}

impl Alphabet {
    // Standard alphabets. Characters are sorted by their ASCII values.
    pub const BASE_2: &'static str = "01";
    pub const BASE_10: &'static str = "0123456789";
    pub const BASE_16: &'static str = "0123456789ABCDEF";
    pub const BASE_62: &'static str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    pub const BASE_64: &'static str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz+/";
    pub const NUMBERS: &'static str = Self::BASE_10;
    pub const HEX: &'static str = Self::BASE_16;
    pub const LETTERS_LOWERCASE: &'static str = "abcdefghijklmnopqrstuvwxyz";
    pub const LETTERS_UPPERCASE: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    pub const LETTERS: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    pub const URL_UNRESERVED_RFC3986: &'static str = "-.0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ_abcdefghijklmnopqrstuvwxyz~";
    pub const ASCII: &'static str = r##" !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~"##;

    pub fn new(chars: Vec<char>) -> Result<Self, Error> {
        let v = Self { chars };
        match v.verify(None) {
            Ok(_) => Ok(v),
            Err(e) => return Err(e),
        }
    }

    pub fn from(string: &str) -> Result<Alphabet, Error> {
        Self::new(string.chars().collect())
    }

    pub fn chars(&self) -> &Vec<char> {
        &self.chars
    }

    pub fn generate(&self, start: i64, count: i64) -> Vec<String> {
        let mut result = Vec::new();
        let alphabet_len = self.chars.len() as i64;
        let mut index = start;
        for _ in start..start + count {
            let mut current = index;
            let mut s = String::new();
            while current >= 0 {
                let remainder = current % alphabet_len;
                s.push(self.chars[remainder as usize] as char);
                current = (current - remainder) / alphabet_len - 1;
            }
            result.push(s.chars().rev().collect());
            index += 1;
        }
        result
    }

    pub fn verify(&self, allowed_chars: Option<Alphabet>) -> Result<(), Error> {
        let mut seen = HashSet::<char>::new();
        let other = match allowed_chars {
            Some(alphabet) => Some(alphabet.chars().clone().into_iter().collect::<HashSet<_>>()),
            None => None,
        };

        for &c in self.chars.iter() {
            // making sure there are no duplicates
            if !seen.insert(c) {
                return Err(Error::DuplicateChar(c));
            }
            // making sure the custom alphabet is a subset of max permissable chars
            if let Some(other) = &other {
                if !other.contains(&c) {
                    return Err(Error::NotSubset(c));
                }
            }
        }
        Ok(())
    }
}

impl Index<usize> for Alphabet {
    type Output = char;

    fn index(&self, index: usize) -> &Self::Output {
        &self.chars[index]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_abcdef() {
        assert_eq!(Alphabet::from("abcdef").unwrap().generate(4, 5), vec!["e", "f", "aa", "ab", "ac"]);
    }

    #[tokio::test]
    async fn test_numbers() {
        let alphabet = Alphabet::from(Alphabet::NUMBERS).unwrap();
        assert_eq!(alphabet.generate(0, 3), vec!["0", "1", "2"]);
        assert_eq!(alphabet.generate(3, 3), vec!["3", "4", "5"]);
        assert_eq!(alphabet.generate(10, 3), vec!["00", "01", "02"]);
    }

    #[tokio::test]
    async fn test_alphabet_generate() {
        let alphabet = Alphabet::new("abc".chars().collect()).unwrap();
        assert_eq!(alphabet.generate(0, 3), vec!["a", "b", "c"]);
        assert_eq!(alphabet.generate(3, 3), vec!["aa", "ab", "ac"]);
        assert_eq!(alphabet.generate(6, 3), vec!["ba", "bb", "bc"]);
        assert_eq!(alphabet.generate(3, 7), vec!["aa", "ab", "ac", "ba", "bb", "bc", "ca"]);
    }

    #[tokio::test]
    #[cfg(feature = "serde")]
    async fn test_ser_de() {
        let alphabet = Alphabet::new("abc".chars().collect()).unwrap();
        let ser = serde_json::to_string(&alphabet).unwrap();
        let de = serde_json::from_str::<Alphabet>(&ser).unwrap();
        assert_eq!(alphabet, de);
    }
}
