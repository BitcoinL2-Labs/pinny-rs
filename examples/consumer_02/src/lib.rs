#[cfg(test)]
mod tests {
    use pinny::tag;

    #[tag(fast)]
    #[test]
    fn test_quick() {
        assert!(true);
    }

    #[tag(slow)]
    #[test]
    fn test_sluggish() {
        assert!(true);
    }
}
