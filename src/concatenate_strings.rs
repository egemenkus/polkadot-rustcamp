/// # Parameters
///
/// `string1,string2` - Program will concatenate two strings
///
/// # Returns
/// string - If passed parameters, returns concatenated sentence
/// Hello, - Rust!  => Hello, Rust!

pub fn concatenate_strings(string1: &str, string2: &str) -> String {
    let mut sentence = String::new();
    sentence.push_str(string1);
    sentence.push_str(string2);
    sentence
}
