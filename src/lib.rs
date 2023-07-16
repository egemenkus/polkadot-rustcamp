pub mod concatenate_strings;
pub mod pythagorean_triples;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::concatenate_strings::concatenate_strings;
    #[test]
    // Pythagorean Triple Test
    fn pythagorean_triple_test() {
        let pythagorean_triple_true = pythagorean_triples::check(Some((3, 4, 5)));
        let pythagorean_triple_false = pythagorean_triples::check(Some((1, 2, 3)));
        assert_eq!(pythagorean_triple_true, true, "9+16==25");
        assert_eq!(pythagorean_triple_false, false, "1+4!=9");
    }

    #[test]
    fn concatenate_strings_test() {
        let string1 = String::from("Hello, ");
        let string2 = String::from("Rust!");
        let actual_string = String::from("Hello, Rust!");
        let test_sentence = concatenate_strings(&string1, &string2);
        assert_eq!(
            test_sentence, actual_string,
            "Sentence must be Hello, Rust!"
        );
    }
}
