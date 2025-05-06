extern crate purua;

fn main() {
    const SOURCE1: &str = r###"
    print("Please fill in here")
"###;
    const SOURCE2: &str = include_str!("../lua_examples/forin.lua");
    let func = |src: &'static str| {
        eprintln!("{}", src);
        let mut scanner = purua::scanner::Scanner::new(src);
        scanner.scan().unwrap();
        let tokens = scanner.tokens;
        dbg!(&tokens);
    
        let stream = purua::parser::stream::TokenStream::new(tokens);
        purua::parser::parser::parse(stream).unwrap();
    };
    func(SOURCE1);
    func(SOURCE2);
}
