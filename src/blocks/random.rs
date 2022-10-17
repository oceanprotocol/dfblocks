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
    while numbers.len() < samples as usize {
        // check if exists
        let number = range.sample(&mut rng);
        if numbers.contains(&number) {
            continue;
        }
        numbers.push(number);
    }
    // sort numbers
    numbers.sort();
    numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_choose() {
        let numbers = random_choose(1, 100, 10);
        assert_eq!(numbers.len(), 10);
        let mut sorted = numbers.clone();
        sorted.sort();

        for n in 0..10 {
            let n = n as usize;
            assert!(numbers[n] == sorted[n]);
        }
    }
}
