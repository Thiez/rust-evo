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
    let mut rng = rand::thread_rng();

    let mut parents = Vec::new();
    for _ in 0..num_parents {
        parents.push(generate_first_sentence(target.len(), &mut rng));
    }

    let rng = RcRng::new(rng);
    let mut f_min = std::u32::MAX;
    let mut sentences: Vec<(u32, Vec<char>)> = Vec::new();
    while f_min != 0 {
        sentences.clear();
        counter+=1;

        sentences.extend(reproduction::generate_children(&parents[..], rng.clone())
            .map(|sentence| mutate(sentence, random_chars(rng.clone()), rng.clone(), mutation_rate).collect::<Vec<_>>())
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

#[derive(Clone)]
struct RcRng<R: Rng>(std::rc::Rc<std::cell::RefCell<R>>);

impl<R: Rng> RcRng<R> {
    fn new(rng: R) -> Self {
        RcRng(std::rc::Rc::new(std::cell::RefCell::new(rng)))
    }
}

impl<R: Rng> Rng for RcRng<R> {
    fn next_u32(&mut self) -> u32 {
        (self.0).borrow_mut().next_u32()
    }
}

/// Computes the fitness of a sentence against a target string.
fn fitness<I1, I2>(target: I1, attempt: I2) -> u32
    where
        I1: IntoIterator,
        <I1 as IntoIterator>::Item: Eq,
        I2: IntoIterator<Item=<I1 as IntoIterator>::Item>,
{
    let mut target = target.into_iter();
    let mut attempt = attempt.into_iter();
    let mut sum = 0;
    loop {
        match (target.next(), attempt.next()) {
            (Some(ref a), Some(ref b)) if a == b => (),
            (None, None) => return sum,
            _ => sum += 1
        }
    }
}

struct MutatedGenes<I1, I2, R>
    where
        I1: Iterator,
        I2: Iterator<Item=<I1 as Iterator>::Item>,
        R: Rng
{
    original: I1,
    mutations: I2,
    rng: R,
    mutation_chance: f64
}

impl<I1, I2, R> Iterator for MutatedGenes<I1, I2, R>
    where
        I1: Iterator,
        I2: Iterator<Item=<I1 as Iterator>::Item>,
        R: Rng
{
    type Item = <I1 as Iterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        let good = self.original.next();
        if good.is_some() && self.rng.gen_range(0.0, 1.0) < self.mutation_chance {
            self.mutations.next()
        } else {
            good
        }
    }
}

fn mutate<I1, I2, R>(original: I1, mutations: I2, rng: R, mutation_chance: f64) ->
    MutatedGenes<<I1 as IntoIterator>::IntoIter, <I2 as IntoIterator>::IntoIter, R>
    where
        I1: IntoIterator,
        I2: IntoIterator<Item=<I1 as IntoIterator>::Item>,
        R: Rng
{
    MutatedGenes {
        original: original.into_iter(),
        mutations: mutations.into_iter(),
        rng: rng,
        mutation_chance: mutation_chance
    }
}

/// Generates a random sentence of length `len` from completly random chars.
fn generate_first_sentence<R: Rng>(len: usize, rng: R) -> Vec<char> {
    random_chars(rng).take(len).collect()
}

struct RandomCharacters<R: Rng> {
    rng: R
}

impl<R: Rng> Iterator for RandomCharacters<R> {
    type Item = char;
    fn next(&mut self) -> Option<char> {
        Some(AVAILABLE_CHARS[self.rng.gen_range(0, AVAILABLE_CHARS.len())])
    }
}

fn random_chars<R: Rng>(rng: R) -> RandomCharacters<R> {
    RandomCharacters { rng: rng }
}

mod reproduction {
    use std::iter::FromIterator;
    use std::ops::Deref;
    use rand::Rng;

    /// An iterator that generates an endless stream of children. See `generate_children`.
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

    /// Create an iterator that produces an endless stream of children.
    /// Each child is produced by picking two random parents (possibly the same parent!)
    /// and grabbing material from either.
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