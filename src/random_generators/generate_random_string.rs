use rand::{distr::Alphanumeric, Rng};

pub fn generate_random_string(num_chars: usize) -> String {
    return rand::rng()
        .sample_iter(&Alphanumeric)
        .take(num_chars)
        .map(char::from)
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use test_case::test_case;

    #[test_case(0 ; "no chars")]
    #[test_case(1 ; "length 1")]
    #[test_case(3 ; "length 3")]
    #[test_case(10 ; "length 10")]
    fn test_produces_correct_length(str_length: usize) {
        let out: String = generate_random_string(str_length);
        assert_eq!(out.len(), str_length);
    }

    #[test]
    fn test_produces_different_outputs() {
        // This might fail randomly but the chances are very small
        let num_tries: usize = 1000;
        let fail_if_less: usize = 995;
        let str_length: usize = 50;

        let unique_strings: HashSet<String> = HashSet::from_iter(
            (0..num_tries)
                .into_iter()
                .map(|_x| generate_random_string(str_length)),
        );
        assert!(unique_strings.len() >= fail_if_less);
    }
}
