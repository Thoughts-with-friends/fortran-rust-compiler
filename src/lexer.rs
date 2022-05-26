use regex::Regex;

/// reference: https://www.sigbus.info/compilerbook#ステップ2加減算のできるコンパイラの作成
pub fn parse(text: &str) {
    let mut p = text.chars().peekable();

    loop {
        if let Some(token) = p.next() {
            match token {
                '+' => {
                    println!("add: {}", token);
                }
                '-' => {
                    println!("sub: {}", token);
                }
                '*' => {
                    println!("mul:{}", token);
                }
                '/' => {
                    println!("div:{}", token);
                }
                _ => match is_number(&token.to_string()) {
                    true => {
                        println!("number: {}", token);
                    }
                    false => {
                        todo!();
                    }
                },
            }
        } else {
            break;
        }
    }
}

fn is_number(string: &str) -> bool {
    let re_num = Regex::new(r"^\d+$").unwrap();
    re_num.is_match(string)
}

#[test]
fn test_number() {
    assert_eq!(true, is_number("1234"));
    assert_eq!(true, is_number("2934"));

    assert_eq!(false, is_number("new1"));
    assert_eq!(false, is_number("12beef334"));
}
