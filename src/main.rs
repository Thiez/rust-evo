//! Author : Thibault Barbie
//!
//! A simple evolutionary algorithm written in Rust.

extern crate rand;

use rand::{Rng};

static AVAILABLE_CHARS: &'static [char] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
    'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', ' '
];

fn main() {
    let target = std::env::args().skip(1).next().unwrap_or("METHINKS IT IS LIKE A WEASEL".into());
    for c in target.chars().filter(|c|!AVAILABLE_CHARS.contains(c)) {
        panic!("Bad character: {}, permissable characters: {}", c, AVAILABLE_CHARS.iter().cloned().collect::<String>());
    }

    println!("{}", target);

    let nb_copy = 400;
    let mutation_rate = 0.05f64;
    let mut counter = 0;
    let num_parents = 3;
    let rng = &mut rand::thread_rng();

    let mut parents = Vec::new();
    for _ in 0..num_parents {
        parents.push(generate_first_sentence(target.len(), rng));
    }

    let mut f_min = std::u32::MAX;
    let mut sentences: Vec<(u32, String)> = Vec::new();
    while f_min != 0 {
        sentences.clear();
        counter+=1;

        for _ in 0..nb_copy {
            let sentence = generate_child(&parents, rng);
            let sentence = mutate(&sentence, mutation_rate, rng);
            let f = fitness(&target, &sentence);
            sentences.push((f,sentence));
        }

        sentences.sort_by_key(|tup|tup.0);
        parents.clear();
        parents.extend(sentences.drain(..).take(num_parents).map(|tup|tup.1));
        let best = &parents[0];
        let new_f_min = fitness(&target, best);
        if new_f_min < f_min {
            f_min = new_f_min;
            println!("{} : {}", best, counter);
        }
    }
}

/// Computes the fitness of a sentence against a target string.
fn fitness(target: &str, sentence: &str) -> u32 {
    target.chars().zip(sentence.chars()).map(|(c1, c2)| if c1 != c2 { 1 } else { 0 }).fold(0, |s, n| s + n)
}

/// Mutation algorithm.
///
/// It mutates each character of a string, according to a `mutation_rate`.
/// Please note that for full usefullness, `mutation_rate` should be between
/// 0 and 1.
fn mutate<R: Rng>(sentence: &str, mutation_rate: f64, rng: &mut R) -> String {
    sentence.chars()
        .map(|c|if mutation_rate < rng.gen_range(0f64, 1.) { c } else { random_char(rng) })
        .collect()
}

/// Generates a random sentence of length `len` from completly random chars.
fn generate_first_sentence<R: Rng>(len: usize, rng: &mut R) -> String {
    let mut result = String::new();
    for _ in 0..len {
        result.push(random_char(rng));
    }

    result
}

/// Generates a random char (between 'A' and '\\').
fn random_char<R: Rng>(rng: &mut R) -> char {
    AVAILABLE_CHARS[rng.gen_range(0, AVAILABLE_CHARS.len())]
}

/// Picks two parents and mixes them together.
fn generate_child<R: Rng>(parents: &[String], rng: &mut R) -> String {
    let parent1 = rng.choose(parents).expect("at leats one parent");
    let parent2 = rng.choose(parents).expect("at least one parent");
    parent1.chars().zip(parent2.chars()).map(|(c1, c2)|*rng.choose(&[c1, c2]).unwrap()).collect()
}