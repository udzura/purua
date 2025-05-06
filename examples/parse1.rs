extern crate purua;

fn main() {
    const SOURCE_: &str = "
    foo[1] = 2
    foo[idx] = 4
    foo.bar = 3
";
    // const SOURCE: &str = include_str!("../lua_examples/fib.lua");
    // const SOURCE: &str = include_str!("../lua_examples/ifthenelse.lua");
    const SOURCE: &str = include_str!("../lua_examples/fib2.lua");
    eprintln!("{}", SOURCE_);
    let mut scanner = purua::scanner::Scanner::new(SOURCE_);
    scanner.scan().unwrap();
    let tokens = scanner.tokens;
    dbg!(&tokens);

    let stream = purua::parser::stream::TokenStream::new(tokens);
    purua::parser::parser::parse(stream).unwrap();
}
