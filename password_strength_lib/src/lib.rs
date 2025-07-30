
mod brute;

pub fn estimate_strength(password: &str) -> f64 {
    brute::basic_brute_force_estimate(password)
}

pub fn brute_force_only(password: &str) -> f64 {
    brute::basic_brute_force_estimate(password)
}
