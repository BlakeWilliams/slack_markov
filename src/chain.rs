extern crate rand;

use std::collections::HashMap;
use self::rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Chain {
    pub map: HashMap<String, Vec<String>>,
    pub gramcount: usize,
}

impl Chain {
    pub fn new(count: usize) -> Chain {
        Chain {
            map: HashMap::with_capacity(8_000_000),
            gramcount: count
        }
    }

    pub fn parse(&mut self, content: &str) {
        let mut buffer = Vec::new();
        let mut word = String::new();

        for character in content.chars() {
            if character == '\n' {
                buffer.push(word);
                if buffer.len() == self.gramcount + 1 {
                    self.insert_pair(&buffer);
                }
                buffer = Vec::new();
                word = String::new();
            } else if character == ' ' {
                buffer.push(word);
                word = String::new();

                if buffer.len() == self.gramcount + 1 {
                    self.insert_pair(&buffer);
                    buffer = buffer.iter().skip(1).map(|s| s.to_string()).collect();
                    word = String::new();
                }
            } else {
                word.push(character);
            }
        }

        if buffer.len() == self.gramcount + 1 {
            self.insert_pair(&buffer);
        }
    }

    fn insert_pair(&mut self, buffer: &Vec<String>) {
        let key_vec = buffer
            .iter()
            .take(self.gramcount)
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let key = key_vec.join(" ");
        let value = buffer.iter().nth(self.gramcount).expect("could not get last value");

        self.insert(key, value.to_string());
    }

    pub fn insert(&mut self, key: String, value: String) {
        if self.map.contains_key(&key) {
            let old_value = self.map.get_mut(&key).unwrap();
            old_value.push(value);
        } else {
            self.map.insert(key, vec!(value));
        }
    }

    pub fn sentence(&self) -> String {
        let key = self.random_key();
        let values = self.map.get(key).expect("could not fetch values");
        let value = random_value(values);

        let mut sentence = format!("{} {}", key, value);

        loop {
            let copy_sentence = sentence.clone();
            let words: Vec<&str> = copy_sentence.split(" ").collect();
            let key = words.iter()
                .skip(words.len() - self.gramcount)
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join(" ");

            match self.map.get(&key) {
                Some(values) => {
                    let value = random_value(values);
                    sentence = format!("{} {}", sentence, value);
                }
                None => break
            }
        }

        sentence
    }

    fn random_key(&self) -> &str {
        let mut rng = thread_rng();

        let keys = self.map.keys().collect::<Vec<&String>>();
        let key = rng.choose(&keys).expect("could not get random key");

        key
    }
}

fn random_value(words: &Vec<String>) -> String {
    let mut rng = thread_rng();
    rng.choose(&words).expect("could not get random word").clone()
}
