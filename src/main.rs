use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
enum Token {
    Number(i32),
    Symbol(String),
    LParen,
    RParen,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Ne,
    Eq,
    Lt,
    Le,
    Gt,
    Ge,
    If,
}

#[derive(Debug)]
struct Env {
    vars: HashMap<String, i32>,
    funs: HashMap<String, (Vec<String>, Vec<Token>)>,  // (params, body)
}

impl Env {
    fn new() -> Self {
        Self {
            vars: HashMap::new(),
            funs: HashMap::new(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            pos: 0,
        }
    }

    fn eof(&mut self) -> bool {
        self.pos >= self.tokens.len()
    }

    fn consume(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }

    fn next(&mut self) -> Option<Token> {
        if let Some(token) = self.tokens.get(self.pos) {
            self.pos += 1;
            Some(token.clone())
        } else {
            None
        }
    }

    fn peek(&self) -> Option<Token> {
        self.tokens.get(self.pos).map(|token| token.clone())
    }
}

fn main() {
    println!("Hello, world!");
}

fn eval(tokens: &mut Parser, env: &mut Env) -> Result<i32, String> {
    if let Some(token) = tokens.next() {
        if token != Token::LParen {
            return Err(format!("expect: '(', but got: '{:?}'", token));
        }
    }

    match tokens.peek() {
        Some(Token::Plus)     |
        Some(Token::Minus)    |
        Some(Token::Asterisk) |
        Some(Token::Slash)    => {
            let op = tokens.next().unwrap();
            let mut sum = match tokens.peek().unwrap() {
                Token::LParen => eval(tokens, env).unwrap(),
                Token::Number(num) => {
                    tokens.consume();
                    num
                }
                Token::Symbol(sym) => {
                    tokens.consume();
                    env.vars[&sym]
                }
                _ => unreachable!()
            };
            let calc = match op {
                Token::Plus     => |sum: i32, num: i32| sum.checked_add(num),
                Token::Minus    => |sum: i32, num: i32| sum.checked_sub(num),
                Token::Asterisk => |sum: i32, num: i32| sum.checked_mul(num),
                Token::Slash    => |sum: i32, num: i32| sum.checked_div(num),
                _ => unreachable!(),
            };
            while let Some(token) = tokens.peek() {
                if token == Token::RParen {
                    tokens.next();
                    break;
                }
                let num = match tokens.peek().unwrap() {
                    Token::LParen => eval(tokens, env).unwrap(),
                    Token::Number(num) => {
                        tokens.consume();
                        num
                    }
                    Token::Symbol(sym) => {
                        tokens.consume();
                        env.vars[&sym]
                    }
                    _ => unreachable!()
                };
                if let Some(result) = calc(sum, num) {
                    sum = result;
                } else {
                    return Err(format!("failed calculation"));
                }
            }
            return Ok(sum);
        }
        Some(Token::Ne) |
        Some(Token::Eq) |
        Some(Token::Lt) |
        Some(Token::Le) |
        Some(Token::Gt) |
        Some(Token::Ge) => {
            let op = tokens.next().unwrap();
            let mut target = match tokens.peek().unwrap() {
                Token::LParen => eval(tokens, env).unwrap(),
                Token::Number(num) => {
                    tokens.consume();
                    num
                }
                Token::Symbol(sym) => {
                    tokens.consume();
                    env.vars[&sym]
                }
                _ => unreachable!()
            };
            let calc = match op {
                Token::Eq => |target: i32, num: i32| if target == num { 1 } else { 0 },
                Token::Ne => |target: i32, num: i32| if target != num { 1 } else { 0 },
                Token::Lt => |target: i32, num: i32| if target <  num { 1 } else { 0 },
                Token::Le => |target: i32, num: i32| if target <= num { 1 } else { 0 },
                Token::Gt => |target: i32, num: i32| if target >  num { 1 } else { 0 },
                Token::Ge => |target: i32, num: i32| if target >= num { 1 } else { 0 },
                _ => unreachable!(),
            };
            while let Some(token) = tokens.peek() {
                if token == Token::RParen {
                    tokens.next();
                    break;
                }
                let num = match tokens.peek().unwrap() {
                    Token::LParen => eval(tokens, env).unwrap(),
                    Token::Number(num) => {
                        tokens.consume();
                        num
                    }
                    Token::Symbol(sym) => {
                        tokens.consume();
                        env.vars[&sym]
                    }
                    _ => unreachable!()
                };
                target = calc(target, num);
            }
            return Ok(target);
        }
        Some(Token::If) => {
            tokens.consume();
            let comp = match tokens.peek().unwrap() {
                Token::LParen => eval(tokens, env).unwrap(),
                Token::Number(num) => {
                    tokens.consume();
                    num
                }
                Token::Symbol(sym) => {
                    tokens.consume();
                    env.vars[&sym]
                }
                _ => unreachable!()
            };
            if comp != 0 {
                let t = match tokens.peek().unwrap() {
                    Token::LParen => eval(tokens, env).unwrap(),
                    Token::Number(num) => {
                        tokens.consume();
                        num
                    }
                    Token::Symbol(sym) => {
                        tokens.consume();
                        env.vars[&sym]
                    }
                    _ => unreachable!()
                };
                // skip nil
                let mut paren_count = 0;
                while let Some(token) = tokens.next() {
                    if token == Token::RParen  && paren_count == 0 {
                        break;
                    }
                    if token == Token::LParen { paren_count += 1; }
                    if token == Token::RParen { paren_count -= 1; }
                }
                return Ok(t);
            } else {
                // skip t
                match tokens.next().unwrap() {
                    Token::LParen => {
                        let mut paren_count = 0;
                        while let Some(token) = tokens.next() {
                            if token == Token::RParen  && paren_count == 0 {
                                break;
                            }
                            if token == Token::LParen { paren_count += 1; }
                            if token == Token::RParen { paren_count -= 1; }
                        }
                    }
                    _ => (),
                }
                let nil = match tokens.peek().unwrap() {
                    Token::LParen => eval(tokens, env).unwrap(),
                    Token::Number(num) => {
                        tokens.consume();
                        num
                    }
                    Token::Symbol(sym) => {
                        tokens.consume();
                        env.vars[&sym]
                    }
                    _ => unreachable!()
                };
                return Ok(nil);
            }
        }
        Some(Token::Symbol(s)) => match s.as_str() {
            // 変数に代入
            "setq" => {
                tokens.consume();
                while let Some(token) = tokens.peek() {
                    if token == Token::RParen {
                        tokens.consume();
                        break;
                    }
                    let var = if let Some(Token::Symbol(var)) = tokens.next() {
                        var
                    } else {
                        panic!("expect variable");
                    };
                    let val = if let Some(Token::LParen) = tokens.peek() {
                        eval(tokens, env).unwrap()
                    } else if let Some(Token::Number(num)) = tokens.next() {
                        num
                    } else {
                        panic!("expect value");
                    };
                    env.vars.insert(var, val);
                }
            }
            // 関数定義
            "defun" => {
                tokens.consume();
                let funcname = if let Some(Token::Symbol(funcname)) = tokens.next() {
                    funcname
                } else {
                    panic!("expect function name");
                };
                if tokens.next().unwrap() != Token::LParen {
                    return Err(format!("expect: '('"));
                }
                let mut params = vec![];
                while let Some(token) = tokens.next() {
                    if token == Token::RParen {
                        break;
                    }
                    if let Token::Symbol(param) = token {
                        params.push(param.to_string());
                    } else {
                        panic!("invalid ident");
                    }
                }
                if tokens.next().unwrap() != Token::LParen {
                    return Err(format!("expect: '('"));
                }
                let mut body = vec![Token::LParen];
                let mut paren_count = 0;
                while let Some(token) = tokens.next() {
                    body.push(token.clone());
                    if token == Token::RParen  && paren_count == 0 {
                        break;
                    }
                    if token == Token::LParen { paren_count += 1; }
                    if token == Token::RParen { paren_count -= 1; }
                }
                env.funs.insert(funcname, (params, body));
                if tokens.next().unwrap() != Token::RParen {
                    return Err(format!("expect: ')'"));
                }
            }
            ident => {
                if let Some(var) = env.vars.get(ident) {
                    // 変数
                    tokens.consume();
                    return Ok(*var);
                } else if env.funs.contains_key(ident) {
                    // 関数呼び出し
                    let (params, body) = env.funs.get(ident).unwrap().clone();
                    tokens.consume();
                    let mut params_iter = params.iter();
                    let mut params_env  = Env::new();
                    while let Some(token) = tokens.peek() {
                        if token == Token::RParen {
                            tokens.consume();
                            break;
                        }
                        // 仮引数に実引数を代入
                        params_env.vars.insert(
                            params_iter.next().unwrap().to_string(),
                            match tokens.peek().unwrap() {
                                Token::LParen => eval(tokens, env).unwrap(),
                                Token::Number(num) => {
                                    tokens.consume();
                                    num
                                }
                                Token::Symbol(sym) => {
                                    tokens.consume();
                                    env.vars[&sym]
                                }
                                _ => unreachable!()
                            }
                        );
                    }
                    params_env.funs = env.funs.clone();
                    return eval(&mut Parser { tokens: body.to_vec(), pos: 0 }, &mut params_env);
                } else {
                    return Err(format!("invalid ident: '{ident}'"));
                }
            }
        }
        Some(Token::Number(n)) => return Ok(n),
        None => (),
        token => return Err(format!("unexpected token: {:?}", token.unwrap())),
    }

    if tokens.eof() {
        Ok(0)
    } else {
        eval(tokens, env)
    }
}

fn tokenize(program: &str) -> Parser {
    Parser::new(program
        .replace('(', " ( ")
        .replace(')', " ) ")
        .split_whitespace()
        .map(|token| match token {
            "("   => Token::LParen,
            ")"   => Token::RParen,
            "+"   => Token::Plus,
            "-"   => Token::Minus,
            "*"   => Token::Asterisk,
            "/"   => Token::Slash,
            "="   => Token::Eq,
            "/="  => Token::Ne,
            "<"   => Token::Lt,
            "<="  => Token::Le,
            ">"   => Token::Gt,
            ">="  => Token::Ge,
            "t"   => Token::Number(1),
            "nil" => Token::Number(0),
            "if"  => Token::If,
            _  => if let Ok(num) = token.parse::<i32>() {
                Token::Number(num)
            } else {
                Token::Symbol(token.to_string())
            }
        })
        .collect()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        assert_eq!(tokenize("(+ 1 2 (- 4 2))"), Parser { tokens: vec![
            Token::LParen,
            Token::Plus,
            Token::Number(1),
            Token::Number(2),
            Token::LParen,
            Token::Minus,
            Token::Number(4),
            Token::Number(2),
            Token::RParen,
            Token::RParen,
        ], pos: 0});
        assert_eq!(tokenize("(setq x 10)"), Parser { tokens: vec![
            Token::LParen,
            Token::Symbol("setq".to_string()),
            Token::Symbol("x".to_string()),
            Token::Number(10),
            Token::RParen,
        ], pos: 0});
    }

    #[test]
    fn test_eval() {
        let mut env = Env::new();

        // 普通の計算式
        assert_eq!(eval(&mut tokenize("(+ 1 2)"), &mut env),   Ok(3));
        assert_eq!(eval(&mut tokenize("(+ 1 2 3)"), &mut env), Ok(6));
        assert_eq!(eval(&mut tokenize("(- 3 2 1)"), &mut env), Ok(0));
        assert_eq!(eval(&mut tokenize("(- 1 2 3)"), &mut env), Ok(-4));
        assert_eq!(eval(&mut tokenize("(* 1 2 3)"), &mut env), Ok(6));
        assert_eq!(eval(&mut tokenize("(/ 8 4 2)"), &mut env), Ok(1));
        assert_eq!(eval(&mut tokenize("(/ 8 4 0)"), &mut env), Err("failed calculation".to_string()));

        // 入れ子
        assert_eq!(eval(&mut tokenize("(+ (* 5 3) 5)"), &mut env), Ok(20));
        assert_eq!(eval(&mut tokenize("(+ (* 5 (- 4 2) 3) 5)"), &mut env), Ok(35));
        assert_eq!(eval(&mut tokenize("(+ (* 5 3 (- 4 2)) 5)"), &mut env), Ok(35));

        // 変数
        assert_eq!(eval(&mut tokenize("(setq x 10) (x)"), &mut env), Ok(10));
        assert_eq!(eval(&mut tokenize("(setq x 42) (* x 2)"), &mut env), Ok(84));
        assert_eq!(eval(&mut tokenize("(setq x 3) (setq y 6) (+ x y)"), &mut env), Ok(9));
        assert_eq!(eval(&mut tokenize("(setq x 3 y 6) (+ x y)"), &mut env), Ok(9));
        assert_eq!(eval(&mut tokenize("(setq x 42 x 21) (x)"), &mut env), Ok(21));
        assert_eq!(eval(&mut tokenize("(setq x 10) (setq y (+ x 100)) (y)"), &mut env), Ok(110));

        // 関数
        assert_eq!(eval(&mut tokenize("(defun add (a b) (+ a b)) (add 1 2)"), &mut env), Ok(3));
        assert_eq!(eval(&mut tokenize("(defun add (a b) (+ a b)) (add (add 1 2) 3)"), &mut env), Ok(6));
        assert_eq!(eval(&mut tokenize("(defun add (a b) (+ a b)) (add 1 (add 2 3))"), &mut env), Ok(6));
        assert_eq!(eval(&mut tokenize("(defun foo (a b c) (+ a (* c 2) b)) (foo 1 2 3)"), &mut env), Ok(9));

        // 比較
        assert_eq!(eval(&mut tokenize("(= 1 2)"),  &mut env), Ok(0));
        assert_eq!(eval(&mut tokenize("(= 2 2)"),  &mut env), Ok(1));
        assert_eq!(eval(&mut tokenize("(/= 1 2)"), &mut env), Ok(1));
        assert_eq!(eval(&mut tokenize("(/= 2 2)"), &mut env), Ok(0));
        assert_eq!(eval(&mut tokenize("(< 1 2)"),  &mut env), Ok(1));
        assert_eq!(eval(&mut tokenize("(< 2 2)"),  &mut env), Ok(0));
        assert_eq!(eval(&mut tokenize("(> 2 2)"),  &mut env), Ok(0));
        assert_eq!(eval(&mut tokenize("(> 2 1)"),  &mut env), Ok(1));
        assert_eq!(eval(&mut tokenize("(<= 1 1)"), &mut env), Ok(1));
        assert_eq!(eval(&mut tokenize("(<= 2 1)"), &mut env), Ok(0));
        assert_eq!(eval(&mut tokenize("(>= 2 2)"), &mut env), Ok(1));
        assert_eq!(eval(&mut tokenize("(>= 1 2)"), &mut env), Ok(0));

        // 分岐
        assert_eq!(eval(&mut tokenize("(if t 10 100)"),   &mut env), Ok(10));
        assert_eq!(eval(&mut tokenize("(if 1 10 100)"),   &mut env), Ok(10));
        assert_eq!(eval(&mut tokenize("(if nil 10 100)"), &mut env), Ok(100));
        assert_eq!(eval(&mut tokenize("(if (+ 1 2) (+ 1 10) (+ 1 100))"),   &mut env), Ok(11));
        assert_eq!(eval(&mut tokenize("(defun fib (x) (if (<= x 1) 1 (+ (fib (- x 1)) (fib (- x 2))))) (fib 9)"), &mut env), Ok(55));
    }
}
