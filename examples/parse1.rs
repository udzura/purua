extern crate purua;

fn main() {
    const SOURCE: &str = "print \"Hello\""; //include_str!("../lua_examples/fib.lua");
    let mut scanner = purua::scanner::Scanner::new(SOURCE);
    scanner.scan().unwrap();
    let tokens = scanner.tokens;
    dbg!(&tokens);

    let stream = purua::parser::stream::TokenStream::new(tokens);
    purua::parser::parser::parse(stream).unwrap();
}
