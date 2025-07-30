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

fn load_dictionary(trie: &mut StrTrie<usize>, path: &PathBuf) {
    let Ok(file) = File::open(path) else {
        return;
    };
    println!(
        "Loading dictionary: {}",
        path.file_name().unwrap().to_string_lossy()
    );
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let Ok(line) = line else {
            continue;
        };
        trie.insert(line.trim().chars(), line.len());
    }
}

#[allow(clippy::cast_precision_loss)]
fn score_span(dict: &StrTrie<usize>, rock_you: &StrTrie<usize>, span: &str) -> f64 {
    println!("scoring span: {span}");
    let num_words = dict.len() as f64;
    let rock_you_count = rock_you.len() as f64;
    let span = demunge(span).to_ascii_lowercase();
    dict.longest_prefix_str(&span).map_or_else(
        || {
            rock_you.longest_prefix_entry_str(&span).map_or_else(
                || {
                    if span.len() == 1 {
                        brute::basic_brute_force_estimate(&span)
                    } else {
                        let (a, b) = span.split_at(1);
                        brute_force_only(a) * score_span(dict, rock_you, b)
                    }
                },
                |(str, found)| {
                    if span.len() == *found {
                        rock_you_count
                    } else if str.len() != *found {
                        93. * score_span(dict, rock_you, span.split_at(1).1)
                    } else {
                        rock_you_count * score_span(dict, rock_you, span.split_at(*found).1)
                    }
                },
            )
        },
        |found| {
            if span.len() == *found {
                num_words
            } else {
                num_words * score_span(dict, rock_you, span.split_at(*found).1)
            }
        },
    )
}

#[must_use]
pub fn estimate_strength(
    password: &str,
    plain_dicts: &[PathBuf],
    rockyou_file: Option<PathBuf>,
) -> f64 {
    let mut dict = StrTrie::<usize>::new();
    for file in plain_dicts {
        load_dictionary(&mut dict, file);
    }

    let mut rock_you = StrTrie::<usize>::new();
    if let Some(file) = rockyou_file {
        load_dictionary(&mut rock_you, &file);
    }
    println!("Finished loading dictionaries");

    let scores = password
        .split_whitespace()
        .map(|span| score_span(&dict, &rock_you, span))
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
