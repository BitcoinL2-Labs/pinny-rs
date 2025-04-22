use pinny::tag;

#[tag(not_existent)]
#[test]
fn test_compilation_failure() {}

fn main() {}
