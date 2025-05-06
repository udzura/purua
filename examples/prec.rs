extern crate purua;
use purua::{parser::ast::*, TokenType};

fn main() {
    const SOURCE1: &str = r###"
    local a = 0 + 1 * 2 + 3 - 4 / 5
"###;
    let func = |src: &'static str| {
        eprintln!("{}", src);
        let mut scanner = purua::scanner::Scanner::new(src);
        scanner.scan().unwrap();
        let tokens = scanner.tokens;
        dbg!(&tokens);

        let stream = purua::parser::stream::TokenStream::new(tokens);
        if let Ok(block) = purua::parser::parser::parse(stream) {
            walk(&block);
        } else {
            eprintln!("Parse error...");
        }
    };
    func(SOURCE1);
}

fn walk(block: &Block) {
    for stat in &block.0.0 {
        match stat {
            Stat::LocalDeclVar(_, explist) => {
                let expr = &explist.as_ref().unwrap().0[0];
                let mut stack: Vec<f64> = Vec::new();
                walk_expr(&mut stack, expr);
                assert!(stack.pop().unwrap() == 4.2);
            }
            _ => { 
                println!("Other stat: {:?}", stat);
            }
        }
    }
}

fn walk_expr(stack: &mut Vec<f64>, expr: &Expr) {
    match expr {
        Expr::ExprBinop(lhs, op, rhs) => {
            walk_expr(stack, lhs);
            walk_expr(stack, rhs);
            let pop1 = stack.pop().unwrap();
            let pop0 = stack.pop().unwrap();
            println!("exit BinOp: {:?} {:?} {:?}", pop0, op.0.token_type, pop1);
            match op.0.token_type {
                TokenType::Plus => stack.push(pop0 + pop1),
                TokenType::Minus => stack.push(pop0 - pop1),
                TokenType::Aster => stack.push(pop0 * pop1),
                TokenType::Slash => stack.push(pop0 / pop1),
                TokenType::Perc => stack.push(pop0 % pop1),
                TokenType::Hat => stack.push(pop0.powf(pop1)),
                _ => panic!("Unknown operator: {:?}", op),
            }
        }
        Expr::Number(v) => {
            println!("Number: {:?}", *v);
            stack.push(*v);
        }
        _ => {
            println!("Other expr: {:?}", expr);
        }
    }
}
