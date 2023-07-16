pub mod pythagorean_triples;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Pythagorean Triple Test
    fn pythagorean_triple_test() {
        let pythagorean_triple_true = pythagorean_triples::check(Some((3, 4, 5)));
        let pythagorean_triple_false = pythagorean_triples::check(Some((1, 2, 3)));
        assert_eq!(pythagorean_triple_true, true, "9+16==25");
        assert_eq!(pythagorean_triple_false, false, "1+4!=9");
    }
}
