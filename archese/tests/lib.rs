extern crate archese;

#[test]
fn it_parses_tokens() {
    assert_eq!(archese::tokenize(")"), [")"]);
    assert_eq!(archese::tokenize("("), ["("]);
}
