fn main()
{
    let tokens = tokenize("4+ 26/ (8- 2)^  4");
    println!("{:?}", tokens);
    println!("{:?}", infixToPostfix(tokens));
}

fn tokenize(string: &str) -> Vec<Token>
{
    let chars: Vec<char> = string.chars().collect();
    let mut tokens: Vec<Token> = Vec::new();

    let mut i = 0;
    while i < chars.len()
    {
        if chars[i].is_whitespace()
        {
            i += 1;
            continue;
        }
        else if chars[i].is_numeric()
        {
            let mut j = i + 1;
            while j < chars.len() && chars[j].is_numeric()
            {
                j += 1;
            }
            let token = Token::Number {
                val: String::from(&string[i..j]),
            };
            tokens.push(token);
            i = j - 1;
        }
        else if chars[i] == '('
        {
            tokens.push(Token::LSep);
        }
        else if chars[i] == ')'
        {
            tokens.push(Token::RSep);
        }
        else
        {
            tokens.push(Token::Operator {
                op: match chars[i]
                {
                    '+' => Operation::Addition,
                    '-' => Operation::Subtraction,
                    '*' => Operation::Multiplication,
                    '/' => Operation::Division,
                    '^' => Operation::Exponentiation,
                    _ => panic!("unknown operator: {}", chars[i]),
                },
            });
        }
        i += 1;
    }

    tokens
}

fn infixToPostfix(mut infixTokens: Vec<Token>) -> Vec<Token>
{
    let mut postfixTokens: Vec<Token> = Vec::new();
    let mut opStack: Vec<Token> = Vec::new();

    for token in infixTokens.drain(..)
    {
        match token
        {
            Token::Number { .. } => postfixTokens.push(token),
            Token::LSep => opStack.push(token),
            Token::RSep =>
            {
                loop
                {
                    let curToken = opStack
                        .pop()
                        .expect("tried to pop an operator off the stack when empty");

                    if curToken == Token::LSep
                    {
                        break;
                    }
                    else
                    {
                        postfixTokens.push(curToken);
                    }
                }
            }
            Token::Operator { .. } =>
            {
                if opStack.len() == 0 || opStack.last().unwrap() == &Token::LSep
                {
                    opStack.push(token)
                }
                else
                {
                    let curOp = match token
                    {
                        Token::Operator { ref op } => op,
                        _ => panic!("token {:?} is not an operator", token),
                    };

                    let stackOp = match opStack
                        .last()
                        .expect("tried to pop operator off empty opStack")
                    {
                        Token::Operator { op } => op,
                        _ => panic!("token {:?} is not an operator", token),
                    };

                    if (curOp.precedence() > stackOp.precedence())
                        || (curOp.precedence() == stackOp.precedence()
                            && curOp.associativity() == Associativity::Right)
                    {
                        opStack.push(token)
                    }
                    else
                    {
                        while opStack.len() != 0
                            && curOp.precedence()
                                < match opStack.last().unwrap()
                                {
                                    Token::Operator { op } => op,
                                    tok => panic!("token {:?} is not an operator", tok),
                                }
                                .precedence()
                            || (curOp.precedence()
                                == match opStack.last().unwrap()
                                {
                                    Token::Operator { op } => op,
                                    tok => panic!("token {:?} is not an operator", tok),
                                }
                                .precedence()
                                && curOp.associativity() == Associativity::Left)
                        {
                            postfixTokens.push(
                                opStack
                                    .pop()
                                    .expect("tried to pop operator off empty opStack"),
                            );
                        }
                        opStack.push(token);
                    }
                }
            }
        }
    }

    while opStack.len() != 0
    {
        postfixTokens.push(opStack.pop().unwrap());
    }

    postfixTokens
}

#[derive(Debug, PartialEq)]
enum Associativity
{
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
enum Operation
{
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Exponentiation,
}

impl Operation
{
    fn str(&self) -> String
    {
        match self
        {
            Self::Addition => "+".to_string(),
            Self::Subtraction => "-".to_string(),
            Self::Multiplication => "*".to_string(),
            Self::Division => "/".to_string(),
            Self::Exponentiation => "^".to_string(),
        }
    }

    fn precedence(&self) -> i32
    {
        match self
        {
            Self::Addition | Self::Subtraction => 2,
            Self::Multiplication | Self::Division => 3,
            Self::Exponentiation => 4,
        }
    }

    fn associativity(&self) -> Associativity
    {
        match self
        {
            Self::Addition | Self::Subtraction | Self::Multiplication | Self::Division =>
            {
                Associativity::Left
            }
            Self::Exponentiation => Associativity::Right,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Token
{
    Number
    {
        val: String,
    },

    Operator
    {
        op: Operation,
    },

    LSep,
    RSep,
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[macro_export]
    macro_rules! create_tokens {
        ($($token:literal),*) => {
            vec![
            $(
                make_token($token),
            )*
            ]
        };
    }

    fn make_num(value: i32) -> Token
    {
        Token::Number { val: value.to_string() }
    }

    fn make_token(string: &str) -> Token
    {
        match string
        {
            "+" => Token::Operator{op: Operation::Addition},
            "-" => Token::Operator{op: Operation::Subtraction},
            "*" => Token::Operator{op: Operation::Multiplication},
            "/" => Token::Operator{op: Operation::Division},
            "^" => Token::Operator{op: Operation::Exponentiation},

            "(" => Token::LSep,
            ")" => Token::RSep,

            number => Token::Number{val: number.to_string()},

        }
    }

    #[test]
    fn test_tokenizer()
    {
        let tokens: Vec<Token> = 
            create_tokens!["4", "+", "26", "/", "(", "8", "-", "2", ")", "^", "4"];

        let test1 = tokenize("4   +  26  /    (8-   2)  ^   4");
        let test2 = tokenize("4+ 26/ (8- 2)^  4");

        assert_eq!(test1, tokens);
        assert_eq!(test2, tokens);

        let tokens = create_tokens![
            "4", "(", "5", "-", "2", ")", "^", "(", "3", "*", "(", "5", "-", "6", ")", ")"
        ];
        let test1 = tokenize("4(5-2)^(3*(5-6))");
        let test2 = tokenize("4 (5-   2   )^(   3 *(    5-  6)  )");

        assert_eq!(tokens, test1);
        assert_eq!(tokens, test2);
    }

    #[test]
    fn test_infix_to_postfix()
    {
        let infix_tokens: Vec<Token> = create_tokens!["5"];
        let postfix_tokens: Vec<Token> = create_tokens!["5"];
        assert_eq!(infixToPostfix(infix_tokens), postfix_tokens);

        let infixTokens: Vec<Token> = create_tokens!["5", "+", "3"];
        let postfixTokens: Vec<Token> = create_tokens!["5", "3", "+"];
        assert_eq!(infixToPostfix(infixTokens), postfixTokens);
    }
}
