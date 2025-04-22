mod test01;
mod test02;

#[cfg(test)]
mod tests {
    use pinny::tag;

    #[tag(fast)]
    #[test]
    #[ignore = "reason"]
    fn test_quick() {
        assert!(true);
    }

    #[tag(fast, bitcoin)]
    #[test]
    fn test_quick_btc() {
        assert!(true);
    }

    #[tag(slow, bitcoin, other)]
    #[test]
    fn test_sluggish_btc() {
        assert!(true);
    }
}
