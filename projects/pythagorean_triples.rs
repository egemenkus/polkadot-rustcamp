extern crate polkadot_rustcamp;
use polkadot_rustcamp::pythagorean_triples;

fn main() {
    pythagorean_triples::check(Some((3, 4, 5)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = pythagorean_triples::check(Some((3, 4, 5)));
        assert_eq!(result, true);
    }
}
