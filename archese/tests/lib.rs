extern crate archese;

#[test]
fn it_parses_tokens() {
    assert_eq!(archese::tokenize(")"), [")"]);
    assert_eq!(archese::tokenize("("), ["("]);
    let program = "(begin (define r 10) (* pi (* r r)))";
    let expected = ["(", "begin", "(", "define", "r", "10", ")", "(", "*", "pi", "(", "*", "r", "r", ")", ")", ")"];
    assert_eq!(archese::tokenize(program), expected);
}
