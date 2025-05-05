extern crate purua;

fn main() {
    const SOURCE: &str = "1 1 1";
    let mut scanner = purua::scanner::Scanner::new(SOURCE);
    scanner.scan().unwrap();
    let tokens = scanner.tokens;
    dbg!(&tokens);

    let stream = purua::parser::stream::TokenStream::new(tokens);
    purua::parser::parser::parse_dummy(stream).unwrap();
}