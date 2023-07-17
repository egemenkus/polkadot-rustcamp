pub mod calculator;
pub mod concatenate_strings;
pub mod pythagorean_triples;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calculator::calculate;
    use crate::concatenate_strings::concatenate_strings;
    use approx::assert_relative_eq;
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

    #[test]
    fn add_test() {
        let operation = calculator::Operation::Add(2.5, 3.7);
        assert_eq!(calculate(operation), 6.2, "2.5+3.7=6.2");
    }
    #[test]
    fn subtract_test() {
        let operation = calculator::Operation::Subtract(8.9, 4.3);
        assert_relative_eq!(calculate(operation), 4.6)
    }
    #[test]
    fn multiply_test() {
        let operation = calculator::Operation::Multiply(5.2, 5.0);
        assert_eq!(calculate(operation), 26.0)
    }
    #[test]
    fn divide_test() {
        let operation = calculator::Operation::Divide(7.2, 3.5);
        // 2.0571428571428574 - 2.05
        assert_relative_eq!(calculate(operation), 2.05, epsilon = 0.01)
    }
}
