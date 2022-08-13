use std::collections::HashMap;

#[derive(Debug)]
struct Env {
    vars: HashMap<String, i32>,
    funs: HashMap<String, (Vec<String>, Vec<String>)>,  // (params, body)
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
    tokens: Vec<String>,
    pos: usize,
}

impl Tokens {
    fn new(tokens: Vec<String>) -> Self {
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

    fn next(&mut self) -> Option<&str> {
        if let Some(token) = self.tokens.get(self.pos) {
            self.pos += 1;
            Some(token)
        } else {
            None
        }
    }

    fn peek(&self) -> Option<&str> {
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
    if tokens.next().unwrap() != "(" {
        return Err(format!("expect: '('"));
    }

    match tokens.peek() {
        Some("+") |
        Some("-") |
        Some("*") |
        Some("/") => {
            let op = tokens.next().unwrap().to_string();
            let mut sum = if let Some("(") = tokens.peek() {
                eval(tokens, env).unwrap()
            } else if let Ok(num) = tokens.peek().unwrap().parse::<i32>() {
                tokens.consume();
                num
            } else {
                env.vars[tokens.next().unwrap()]
            };
            let calc = match &*op {
                    "+" => |sum: i32, num: i32| sum.checked_add(num),
                    "-" => |sum: i32, num: i32| sum.checked_sub(num),
                    "*" => |sum: i32, num: i32| sum.checked_mul(num),
                    "/" => |sum: i32, num: i32| sum.checked_div(num),
                    _  => unreachable!(),
            };
            while let Some(token) = tokens.peek() {
                if token == ")" {
                    tokens.next();
                    break;
                }
                let num = if token == "(" {
                    eval(tokens, env).unwrap()
                } else if let Ok(num) = tokens.peek().unwrap().parse::<i32>() {
                    tokens.consume();
                    num
                } else {
                    env.vars[tokens.next().unwrap()]
                };
                if let Some(result) = calc(sum, num) {
                    sum = result;
                } else {
                    return Err(format!("failed calculation"));
                }
            }
            return Ok(sum);
        }
        Some(token) => {
            match token {
                // 変数に代入
                "setq" => {
                    tokens.consume();
                    while let Some(token) = tokens.peek() {
                        if token == ")" {
                            tokens.consume();
                            break;
                        }
                        let var = tokens
                            .next()
                            .expect("expect variable")
                            .to_string();
                        let val = if let Some("(") = tokens.peek() {
                            eval(tokens, env).unwrap()
                        } else {
                            tokens
                                .next()
                                .expect("expect value")
                                .parse::<i32>()
                                .unwrap()
                        };
                        env.vars.insert(var, val);
                    }
                }
                // 関数定義
                "defun" => {
                    tokens.consume();
                    let funcname = tokens
                        .next()
                        .expect("function name")
                        .to_string();
                    if tokens.next().unwrap() != "(" {
                        return Err(format!("expect: '('"));
                    }
                    let mut params = vec![];
                    while let Some(token) = tokens.next() {
                        if token == ")" {
                            break;
                        }
                        params.push(token.to_string());
                    }
                    if tokens.next().unwrap() != "(" {
                        return Err(format!("expect: '('"));
                    }
                    let mut body = vec!["(".to_string()];
                    let mut paren_count = 0;
                    while let Some(token) = tokens.next() {
                        body.push(token.to_string());
                        if token == ")" && paren_count == 0 {
                            break;
                        }
                        if token == "(" { paren_count += 1; }
                        if token == ")" { paren_count -= 1; }
                    }
                    env.funs.insert(funcname, (params, body));
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
                            if token == ")" {
                                tokens.consume();
                                break;
                            }
                            // 仮引数に実引数を代入
                            params_env.vars.insert(
                                params_iter.next().unwrap().to_string(),
                                if token == "(" {
                                    eval(tokens, env).unwrap()
                                } else if let Ok(num) = token.parse::<i32>() {
                                    tokens.consume();
                                    num
                                } else {
                                    let var = env.vars[token];
                                    tokens.consume();
                                    var
                                }
                            );
                        }
                        return eval(&mut Tokens { tokens: body, pos: 0 }, &mut params_env);
                    } else {
                        unimplemented!()
                    }
                }
            }
        }
        None => (),
    }

    if tokens.eof() {
        Ok(0)
    } else {
        eval(tokens, env)
    }
}

fn tokenize(expr: &str) -> Tokens {
    let replaced_expr = expr
        .replace('(', " ( ")
        .replace(')', " ) ");
    Tokens::new(
        replaced_expr
            .split_whitespace()
            .map(|token| token.to_string())
            .collect()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        assert_eq!(tokenize("(+ 1 2 (- 4 2))"), Tokens { tokens: vec![
            "(".to_string(),
            "+".to_string(),
            "1".to_string(),
            "2".to_string(),
            "(".to_string(),
            "-".to_string(),
            "4".to_string(),
            "2".to_string(),
            ")".to_string(),
            ")".to_string(),
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
