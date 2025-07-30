use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use itertools::Itertools;
use rstrie::StrTrie;

mod brute;

fn demunge(password: &str) -> String {
    password
        .replace(['@', '4'], "a")
        .replace('8', "b")
        .replace('(', "c")
        .replace('6', "d")
        .replace('3', "e")
        .replace('9', "g")
        .replace(['1', '!'], "i")
        .replace('0', "o")
        .replace('2', "r")
        .replace(['5', '$'], "s")
        .replace('+', "t")
        .replace(['<', '>'], "v")
        .replace('%', "x")
        .replace('?', "y")
}

fn load_dictionary(trie: &mut StrTrie<usize>, path: PathBuf) -> usize {
    let Ok(file) = File::open(path) else {
        return 0;
    };
    let reader = BufReader::new(file);

    let mut count = 0;
    for line in reader.lines() {
        let Ok(line) = line else {
            continue;
        };
        trie.insert(line.trim().chars(), line.len());
        count += 1;
    }
    count
}

#[must_use]
pub fn estimate_strength(
    password: &str,
    plain_dicts: Vec<PathBuf>,
    rockyou_file: Option<PathBuf>,
) -> f64 {
    let mut dict = StrTrie::<usize>::new();
    let mut num_words = 0;
    for file in plain_dicts {
        num_words += load_dictionary(&mut dict, file);
    }
    let mut rock_you = StrTrie::<usize>::new();
    let mut rock_you_count = 0;
    if let Some(file) = rockyou_file {
        rock_you_count += load_dictionary(&mut rock_you, file);
    }
    #[allow(clippy::cast_precision_loss)]
    let num_words = num_words as f64;
    #[allow(clippy::cast_precision_loss)]
    let rock_you_count = rock_you_count as f64;
    let scores = password
        .split_whitespace()
        .map(demunge)
        .map(|span| {
            dict.longest_prefix_str(&span.to_ascii_lowercase())
                .map_or_else(
                    || {
                        rock_you
                            .longest_prefix_str(&span.to_ascii_lowercase())
                            .map_or_else(
                                || brute::basic_brute_force_estimate(&span),
                                |found| {
                                    if span.len() == *found {
                                        rock_you_count
                                    } else {
                                        rock_you_count
                                            * brute::basic_brute_force_estimate(
                                                span.split_at(*found).1,
                                            )
                                    }
                                },
                            )
                    },
                    |found| {
                        if span.len() == *found {
                            num_words
                        } else {
                            num_words * brute::basic_brute_force_estimate(span.split_at(*found).1)
                        }
                    },
                )
        })
        .collect_vec();
    if scores.len() == 1 {
        scores[0]
    } else {
        let mut result = 1.;
        for score in scores {
            result *= score;
        }
        result
    }
}

#[must_use]
pub fn brute_force_only(password: &str) -> f64 {
    brute::basic_brute_force_estimate(password)
}
