extern crate purua;

fn main() {
    // const SOURCE: &str = "print(\"Hello\")";
    // const SOURCE: &str = include_str!("../lua_examples/fib.lua");
    const SOURCE: &str = "nth = 10
-- ret = fib(nth)
print(ret)
print(\"\\n\")
return 1";
    eprintln!("{}", SOURCE);
    let mut scanner = purua::scanner::Scanner::new(SOURCE);
    scanner.scan().unwrap();
    let tokens = scanner.tokens;
    dbg!(&tokens);

    let stream = purua::parser::stream::TokenStream::new(tokens);
    purua::parser::parser::parse(stream).unwrap();
}
