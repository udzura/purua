extern crate purua;

fn main() {
    const SOURCE_: &str = "
function dofib(n)
   if n < 2 then
      return 1
   else
      v = dofib(n-1) + dofib(n-2)
      return v
   end
   print(\"Unreachable!\\n\")
end

print(dofib(25))
";
    // const SOURCE: &str = include_str!("../lua_examples/fib.lua");
    // const SOURCE: &str = include_str!("../lua_examples/ifthenelse.lua");
    const SOURCE: &str = include_str!("../lua_examples/fib2.lua");
    eprintln!("{}", SOURCE);
    let mut scanner = purua::scanner::Scanner::new(SOURCE);
    scanner.scan().unwrap();
    let tokens = scanner.tokens;
    dbg!(&tokens);

    let stream = purua::parser::stream::TokenStream::new(tokens);
    purua::parser::parser::parse(stream).unwrap();
}
