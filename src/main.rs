extern crate purua;

fn main() {
    let code = "-- comment ignored:
if 1 + 1 < 2 do
  print(\"hello, world\")
  return 3
end
        ";
    dbg!(code);
    let mut s = purua::scanner::Scanner::new(code);
    s.scan().unwrap();

    dbg!(s.tokens);
}
