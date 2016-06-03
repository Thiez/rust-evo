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
    let target: Vec<char> = std::env::args().skip(1).next()
        .map(|s|s.chars().collect())
        .unwrap_or("METHINKS IT IS LIKE A WEASEL".chars().collect());
    for c in target.iter().filter(|c|!AVAILABLE_CHARS.contains(c)) {
        panic!("Bad character: {}, permissable characters: {}", c, AVAILABLE_CHARS.iter().cloned().collect::<String>());
    }

    println!("{}", target.iter().cloned().collect::<String>());

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
    let mut sentences: Vec<(u32, Vec<char>)> = Vec::new();
    while f_min != 0 {
        sentences.clear();
        counter+=1;

        sentences.extend(reproduction::generate_children(&parents[..], rng.clone())
            .map(|sentence| mutate(&sentence, mutation_rate, rng))
            .map(|sentence| (fitness(&target, &sentence), sentence))
            .take(nb_copy));

        sentences.sort_by_key(|tup|tup.0);
        parents.clear();
        parents.extend(sentences.drain(..).take(num_parents).map(|tup|tup.1));
        let best = &parents[0];
        let new_f_min = fitness(&target, best);
        if new_f_min < f_min {
            f_min = new_f_min;
            println!("{} : {}", best.iter().cloned().collect::<String>(), counter);
        }
    }
}

/// Computes the fitness of a sentence against a target string.
fn fitness(target: &[char], sentence: &[char]) -> u32 {
    target.iter().zip(sentence.iter()).filter(|&(&c1, &c2)|c1 != c2).count() as u32
}

/// Mutation algorithm.
///
/// It mutates each character of a string, according to a `mutation_rate`.
/// Please note that for full usefullness, `mutation_rate` should be between
/// 0 and 1.
fn mutate<R: Rng>(sentence: &[char], mutation_rate: f64, rng: &mut R) -> Vec<char> {
    sentence.iter()
        .map(|&c|if mutation_rate < rng.gen_range(0f64, 1.) { c } else { random_char(rng) })
        .collect()
}

/// Generates a random sentence of length `len` from completly random chars.
fn generate_first_sentence<R: Rng>(len: usize, rng: &mut R) -> Vec<char> {
    let mut result = Vec::new();
    for _ in 0..len {
        result.push(random_char(rng));
    }

    result
}

/// Generates a random char (between 'A' and '\\').
fn random_char<R: Rng>(rng: &mut R) -> char {
    AVAILABLE_CHARS[rng.gen_range(0, AVAILABLE_CHARS.len())]
}

mod reproduction {
    use std::iter::FromIterator;
    use std::ops::Deref;
    use rand::Rng;

    pub struct Children<'a, T, R>
        where
            &'a T: IntoIterator,
            <&'a T as IntoIterator>::Item: Deref,
            <<&'a T as IntoIterator>::Item as Deref>::Target: Clone,
            T: 'a + FromIterator<<<&'a T as IntoIterator>::Item as Deref>::Target>,
            R: 'a + Rng,
    {
        parents: &'a [T],
        rng: R,
    }

    impl<'a, T, R> Iterator for Children<'a, T, R>
        where
            &'a T: IntoIterator,
            <&'a T as IntoIterator>::Item: Deref,
            <<&'a T as IntoIterator>::Item as Deref>::Target: Clone,
            T: 'a + FromIterator<<<&'a T as IntoIterator>::Item as Deref>::Target>,
            R: 'a + Rng,
    {
        type Item = T;
        fn next(&mut self) -> Option<Self::Item> {
            let parent1 = &self.parents[self.rng.gen_range(0, self.parents.len())];
            let parent2 = &self.parents[self.rng.gen_range(0, self.parents.len())];
            let child = parent1.into_iter()
                .zip(parent2.into_iter())
                .map(|(e1, e2)|if self.rng.gen() { e1.clone() } else { e2.clone() })
                .collect();
            Some(child)
        }
    }

    pub fn generate_children<'a, T, R>(parents: &'a [T], rng: R) -> Children<'a, T, R>
        where
            &'a T: IntoIterator,
            <&'a T as IntoIterator>::Item: Deref,
            <<&'a T as IntoIterator>::Item as Deref>::Target: Clone,
            T: 'a + FromIterator<<<&'a T as IntoIterator>::Item as Deref>::Target>,
            R: 'a + Rng,
    {
        Children {
            parents: parents,
            rng: rng
        }
    }
}