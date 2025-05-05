extern crate purua;

fn main() {
    const SOURCE: &str = "foo = 1
    bar, buz, quz = 2, 3.5, \"hoge\"
    return 3";
    let mut scanner = purua::scanner::Scanner::new(SOURCE);
    scanner.scan().unwrap();
    let tokens = scanner.tokens;
    dbg!(&tokens);

    let stream = purua::parser::stream::TokenStream::new(tokens);
    purua::parser::parser::parse(stream).unwrap();
}