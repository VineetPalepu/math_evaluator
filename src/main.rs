fn main()
{
    let tokens = tokenize("4+ 26/ (8- 2)^  4");
    println!("{:?}", tokens);
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
            let token = Token {
                data: String::from(&string[i..j]),
            };
            tokens.push(token);
            i = j - 1;
        }
        else
        {
            tokens.push(Token {
                data: String::from(chars[i]),
            })
        }
        i += 1;
    }

    tokens
}

fn infixToPostfix(mut infixTokens: Vec<TokenType>) -> Vec<TokenType>
{
    let mut postfixTokens: Vec<TokenType> = Vec::new();
    let mut opStack: Vec<TokenType> = Vec::new();

    for token in infixTokens.drain(..)
    {
        match token
        {
            TokenType::Number { .. } => postfixTokens.push(token),
            TokenType::Operator { ref op } if op.str() == "(" => opStack.push(token),
            TokenType::Operator { ref op } if op.str() == ")" =>
            {
                loop
                {
                    let curToken = opStack
                        .pop()
                        .expect("tried to pop an operator off the stack when empty");

                    match curToken
                    {
                        TokenType::Operator { op: stackOp } if stackOp.str() == "(" => break,
                        _ => (postfixTokens.push(curToken)),
                    }
                }
            }
            TokenType::Operator { .. } =>
            {
                if opStack.len() == 0
                    || match opStack.last().unwrap()
                    {
                        TokenType::Operator { ref op } => op.str(),
                        _ => panic!(
                            "operator stack contains non-operator token: {:?}",
                            opStack.last().unwrap()
                        ),
                    } == "("
                {
                    opStack.push(token)
                }
                else
                {
                    let curOp = match token
                    {
                        TokenType::Operator { ref op } => op,
                        _ => panic!("token {:?} is not an operator", token),
                    };

                    let stackOp = match opStack
                        .last()
                        .expect("tried to pop operator off empty opStack")
                    {
                        TokenType::Operator { op } => op,
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
                                    TokenType::Operator { op } => op,
                                    tok => panic!("token {:?} is not an operator", tok),
                                }
                                .precedence()
                            || (curOp.precedence()
                                == match opStack.last().unwrap()
                                {
                                    TokenType::Operator { op } => op,
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
enum TokenType
{
    Number
    {
        val: String
    },

    Operator
    {
        op: Operation
    },
}

#[derive(Debug, PartialEq)]
struct Token
{
    data: String,
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
                Token{
                    data: $token.to_string(),
                },
            )*
            ]
        };
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
        let infixTokens: Vec<TokenType> = vec![TokenType::Number {
            val: "5".to_string(),
        }];
        assert_eq!(
            infixToPostfix(infixTokens),
            vec![TokenType::Number {
                val: "5".to_string()
            }]
        );

        let infixTokens: Vec<TokenType> = vec![
            TokenType::Number {
                val: "5".to_string(),
            },
            TokenType::Operator {
                op: Operation::Addition,
            },
            TokenType::Number {
                val: "3".to_string(),
            },
        ];
        let postfixTokens: Vec<TokenType> = vec![
            TokenType::Number {
                val: "5".to_string(),
            },
            TokenType::Number {
                val: "3".to_string(),
            },
            TokenType::Operator {
                op: Operation::Addition,
            },
        ];
        assert_eq!(infixToPostfix(infixTokens), postfixTokens);
    }
}
