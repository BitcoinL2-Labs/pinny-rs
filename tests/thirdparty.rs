#[macro_use]
mod utils;

mod thirdparty_tokio {

    use pinny::tag;
    use tokio::test;

    #[tag(tag1)]
    #[test]
    async fn test_tokio_crate() {
        assert_eq!(
            "thirdparty_tokio::test_tokio_crate::t::tag1::t::{{closure}}",
            function_path!()
        );
    }
}
