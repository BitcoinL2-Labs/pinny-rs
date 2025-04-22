#[cfg(test)]
mod tests {
    use pinny::tag;

    #[tag(fast)]
    #[test]
    fn test_01_quick() {
        assert!(true);
    }

    #[tag(slow, bitcoin)]
    #[test]
    fn test_01_notquick() {
        assert!(true);
    }
}
