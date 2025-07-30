fn get_char_range(span: &str) -> f64 {
    let mut char_range = 0.;
    let mut num_range = 0.;
    if span.chars().any(char::is_alphabetic) {
        if span.eq(&span.to_ascii_lowercase()) {
            char_range = 26.;
        } else {
            char_range = 26. * 2.;
        }
    }
    if span.chars().any(|c| c == ' ') {
        char_range += 1.;
    }
    if span.chars().any(|c| c.is_ascii_punctuation()) {
        char_range += 30.;
    }
    if span.chars().any(char::is_numeric) {
        num_range = 10.;
    }
    char_range + num_range
}

pub fn basic_brute_force_estimate(password: &str) -> f64 {
    // The unwrap is acceptable here as the max password size prevents this from being an issue
    get_char_range(password).powi(password.len().try_into().unwrap())
}
