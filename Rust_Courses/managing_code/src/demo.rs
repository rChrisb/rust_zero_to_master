fn all_caps(word: &str) -> String {
    word.to_uppercase()
}
fn main() {}
#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn check_all_caps() {
        let result = all_caps("bonjour");
        let expected = "BONJOUR".to_owned();
        assert_eq!(result, expected, "It should be all uppercase")
    }
}