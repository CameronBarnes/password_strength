use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use itertools::Itertools;
use rstrie::StrTrie;

mod brute;

// I dont currently like how the demunging is behaving
/*fn demunge(password: &str) -> String {
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
}*/

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
        let line = line.to_ascii_lowercase();
        let line = line.trim();
        trie.insert(line.chars(), line.len());
    }
}

fn load_dictionaries(dict_paths: &[PathBuf]) -> Vec<StrTrie<usize>> {
    let mut out = Vec::new();
    for path in dict_paths {
        let mut dict = StrTrie::<usize>::new();
        load_dictionary(&mut dict, path);
        out.push(dict);
    }
    out.push(load_small_num_dict());
    out.sort_unstable_by_key(rstrie::Trie::len);
    out
}

#[allow(clippy::cast_precision_loss)]
fn score_span(dicts: &[StrTrie<usize>], span: &str) -> f64 {
    if span.len() == 1 {
        return brute_force_only(span);
    }
    // let span = demunge(span).to_ascii_lowercase();
    let span = span.to_ascii_lowercase();
    println!("span: {span}");
    for dict in dicts {
        if let Some((found, _length)) = dict.longest_prefix_entry_str(&span) {
            if span.eq_ignore_ascii_case(&found) {
                // println!("found: {found}");
                return dict.len() as f64;
            } else if let Some((_before, after)) = span.split_once(&found) {
                // println!("good prefix found: {found}");
                return dict.len() as f64 * score_span(dicts, after);
            }
        }
    }
    // println!("None found, brute forcing span: {span}");
    brute_force_only(&span)
}

fn load_small_num_dict() -> StrTrie<usize> {
    println!("Loading dictionary: small numbers");
    let mut small_num_dict = StrTrie::<usize>::new();
    for i in 100..=2100 {
        let str = i.to_string();
        let str = str.trim();
        small_num_dict.insert(str.chars(), str.len());
    }
    for i in 2..=9 {
        let str = (1111 * i).to_string();
        let str = str.trim();
        small_num_dict.insert(str.chars(), str.len());
    }
    small_num_dict
}

#[must_use]
pub fn estimate_strength(password: &str, dict_files: &[PathBuf]) -> f64 {
    let dicts = load_dictionaries(dict_files);
    println!("Finished loading dictionaries");

    let scores = password
        .split_whitespace()
        .map(|span| score_span(&dicts, span))
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
