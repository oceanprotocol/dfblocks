use rand::{
    distributions::{Distribution, Uniform},
    SeedableRng,
};

pub fn random_choose(start: u64, end: u64, samples: u64) -> Vec<u64> {
    // read SECRET_SEED from env
    let secret_seed = std::env::var("SECRET_SEED").unwrap_or("0".to_string());
    let secret_seed = secret_seed.parse::<u64>().unwrap();
    let mut rng = rand::rngs::StdRng::seed_from_u64(secret_seed);
    let range = Uniform::from(start..end);
    let mut numbers = Vec::new();
    for _ in 0..samples {
        // check if exists
        let number = range.sample(&mut rng);
        if numbers.contains(&number) {
            continue;
        }
        numbers.push(number);
    }
    numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_choose() {
        let numbers = random_choose(1, 100, 10);
        assert_eq!(numbers.len(), 10);
    }
}
