use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
enum Token {
    Number(i32),
    Symbol(String),
    LParen,
    RParen,
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
struct Tokens {
    tokens: Vec<Token>,
    pos: usize,
}

impl Tokens {
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

    fn next(&mut self) -> Option<&Token> {
        if let Some(token) = self.tokens.get(self.pos) {
            self.pos += 1;
            Some(token)
        } else {
            None
        }
    }

    fn peek(&self) -> Option<&Token> {
        if let Some(token) = self.tokens.get(self.pos) {
            Some(token)
        } else {
            None
        }
    }
}

fn main() {
    println!("Hello, world!");
}

fn eval(tokens: &mut Tokens, env: &mut Env) -> Result<i32, String> {
    if *tokens.next().unwrap() != Token::LParen {
        return Err(format!("expect: '('"));
    }

    match tokens.peek() {
        Some(Token::Symbol(s)) => match s.as_str() {
            "+" | "-" | "*" | "/" => {
                tokens.consume();
                let mut sum = match tokens.peek().unwrap() {
                    Token::LParen => eval(tokens, env).unwrap(),
                    Token::Number(num) => {
                        tokens.consume();
                        *num
                    }
                    Token::Symbol(sym) => {
                        tokens.consume();
                        env.vars[sym]
                    }
                    _ => unreachable!()
                };
                let calc = match s.as_str() {
                    "+" => |sum: i32, num: i32| sum.checked_add(num),
                    "-" => |sum: i32, num: i32| sum.checked_sub(num),
                    "*" => |sum: i32, num: i32| sum.checked_mul(num),
                    "/" => |sum: i32, num: i32| sum.checked_div(num),
                    _  => unreachable!(),
                };
                while let Some(token) = tokens.peek() {
                    if *token == Token::RParen {
                        tokens.next();
                        break;
                    }
                    let num = match tokens.peek().unwrap() {
                        Token::LParen => eval(tokens, env).unwrap(),
                        Token::Number(num) => {
                            tokens.consume();
                            *num
                        }
                        Token::Symbol(sym) => {
                            tokens.consume();
                            env.vars[sym]
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
            // 変数に代入
            "setq" => {
                tokens.consume();
                while let Some(token) = tokens.peek() {
                    if *token == Token::RParen {
                        tokens.consume();
                        break;
                    }
                    let var = if let Some(Token::Symbol(var)) = tokens.next() {
                        &*var
                    } else {
                        panic!("expect variable");
                    };
                    let val = if let Some(Token::LParen) = tokens.peek() {
                        eval(tokens, env).unwrap()
                    } else if let Some(Token::Number(num)) = tokens.next() {
                        *num
                    } else {
                        panic!("expect value");
                    };
                    env.vars.insert(*var, val);
                }
            }
            // 関数定義
            "defun" => {
                tokens.consume();
                let funcname = if let Some(Token::Symbol(funcname)) = tokens.next() {
                    &*funcname
                } else {
                    panic!("expect function name");
                };
                if *tokens.next().unwrap() != Token::LParen {
                    return Err(format!("expect: '('"));
                }
                let mut params = vec![];
                while let Some(token) = tokens.next() {
                    if *token == Token::RParen {
                        break;
                    }
                    if let Token::Symbol(param) = token {
                        params.push(param.to_string());
                    } else {
                        panic!("invalid ident");
                    }
                }
                if *tokens.next().unwrap() != Token::LParen {
                    return Err(format!("expect: '('"));
                }
                let mut body = vec![Token::LParen];
                let mut paren_count = 0;
                while let Some(token) = tokens.next() {
                    body.push(*token);
                    if *token == Token::RParen  && paren_count == 0 {
                        break;
                    }
                    if *token == Token::LParen { paren_count += 1; }
                    if *token == Token::RParen { paren_count -= 1; }
                }
                env.funs.insert(funcname.to_string(), (params, body));
            }
            ident => {
                if let Some(var) = env.vars.get(ident) {
                    // 変数
                    tokens.consume();
                    return Ok(*var);
                } else if env.funs.contains_key(ident) {
                    // 関数呼び出し
                    let (params, body) = env.funs.get(ident).unwrap();
                    tokens.consume();
                    let mut params_iter = params.iter();
                    let mut params_env  = Env::new();
                    while let Some(token) = tokens.peek() {
                        if *token == Token::RParen {
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
                                    *num
                                }
                                Token::Symbol(sym) => {
                                    tokens.consume();
                                    env.vars[sym]
                                }
                                _ => unreachable!()
                            }
                        );
                    }
                    return eval(&mut Tokens { tokens: body.to_vec(), pos: 0 }, &mut params_env);
                } else {
                    unimplemented!()
                }
            }
        }
        Some(Token::Number(n)) => return Ok(*n),
        None => (),
        token => return Err(format!("unexpected token: {:?}", token.unwrap())),
    }

    if tokens.eof() {
        Ok(0)
    } else {
        eval(tokens, env)
    }
}

fn tokenize(program: &str) -> Tokens {
    Tokens::new(program
        .replace('(', " ( ")
        .replace(')', " ) ")
        .split_whitespace()
        .map(|token| match token {
            "(" => Token::LParen,
            ")" => Token::RParen,
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
        assert_eq!(tokenize("(+ 1 2 (- 4 2))"), Tokens { tokens: vec![
            Token::LParen,
            Token::Symbol("+".to_string()),
            Token::Symbol("1".to_string()),
            Token::Symbol("2".to_string()),
            Token::LParen,
            Token::Symbol("-".to_string()),
            Token::Symbol("4".to_string()),
            Token::Symbol("2".to_string()),
            Token::RParen,
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
        assert_eq!(eval(&mut tokenize("(defun add (a b) (+ a b) (add 1 2)"), &mut env), Ok(3));
        assert_eq!(eval(&mut tokenize("(defun add (a b) (+ a b) (add (add 1 2) 3)"), &mut env), Ok(6));
        assert_eq!(eval(&mut tokenize("(defun add (a b) (+ a b) (add 1 (add 2 3))"), &mut env), Ok(6));
        assert_eq!(eval(&mut tokenize("(defun foo (a b c) (+ a (* c 2) b) (foo 1 2 3)"), &mut env), Ok(9));
    }
}
