#[cfg(test)]
mod tests {
    use pinny::tag;

    #[tag(other)]
    #[test]
    fn test_01_quick() {
        assert!(true);
    }

    #[tag(other, bitcoin)]
    #[test]
    fn test_01_notquick() {
        assert!(true);
    }
}
