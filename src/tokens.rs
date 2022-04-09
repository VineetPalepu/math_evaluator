#[derive(Debug, PartialEq)]
pub enum Token
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

impl Token
{
    pub fn get_number(&self) -> Option<&String>
    {
        match self
        {
            Token::Number { val } => Some(val),
            _ => None,
        }
    }

    pub fn get_operator(&self) -> Option<&Operation>
    {
        match self
        {
            Token::Operator { op } => Some(op),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Associativity
{
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
pub enum Operation
{
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Exponentiation,
}

impl Operation
{
    pub fn str(&self) -> String
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

    pub fn precedence(&self) -> i32
    {
        match self
        {
            Self::Addition | Self::Subtraction => 2,
            Self::Multiplication | Self::Division => 3,
            Self::Exponentiation => 4,
        }
    }

    pub fn associativity(&self) -> Associativity
    {
        match self
        {
            #[rustfmt::skip]
            Self::Addition | Self::Subtraction | Self::Multiplication | Self::Division => Associativity::Left,
            Self::Exponentiation => Associativity::Right,
        }
    }
}

pub fn tokenize(string: &str) -> Vec<Token>
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
        else if chars[i].is_numeric() || chars[i] == '.'
        {
            let mut j = i + 1;
            while j < chars.len() && (chars[j].is_numeric() || chars[j] == '.')
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

pub fn shunting_yard(infix_tokens: Vec<Token>) -> Vec<Token>
{
    let mut postfix_tokens: Vec<Token> = Vec::new();
    let mut op_stack: Vec<Token> = Vec::new();

    for token in infix_tokens
    {
        match token
        {
            Token::Number { .. } => postfix_tokens.push(token),
            Token::LSep => op_stack.push(token),
            Token::RSep =>
            {
                loop
                {
                    let cur_token = op_stack
                        .pop()
                        .expect("tried to pop an operator off the stack when empty");

                    if cur_token == Token::LSep
                    {
                        break;
                    }
                    else
                    {
                        postfix_tokens.push(cur_token);
                    }
                }
            },
            Token::Operator { .. } =>
            {
                if op_stack.is_empty() || op_stack.last().unwrap() == &Token::LSep
                {
                    op_stack.push(token)
                }
                else
                {
                    let cur_op = match token
                    {
                        Token::Operator { ref op } => op,
                        _ => panic!("token {:?} is not an operator", token),
                    };

                    let stack_op = match op_stack
                        .last()
                        .expect("tried to pop operator off empty opStack")
                    {
                        Token::Operator { op } => op,
                        _ => panic!("token {:?} is not an operator", token),
                    };

                    if (cur_op.precedence() > stack_op.precedence())
                        || (cur_op.precedence() == stack_op.precedence()
                            && cur_op.associativity() == Associativity::Right)
                    {
                        op_stack.push(token)
                    }
                    else
                    {
                        while !op_stack.is_empty()
                            && cur_op.precedence()
                                < match op_stack.last().unwrap()
                                {
                                    Token::Operator { op } => op,
                                    tok => panic!("token {:?} is not an operator", tok),
                                }
                                .precedence()
                            && (cur_op.precedence()
                                == match op_stack.last().unwrap()
                                {
                                    Token::Operator { op } => op,
                                    tok => panic!("token {:?} is not an operator", tok),
                                }
                                .precedence()
                                && cur_op.associativity() == Associativity::Left)
                        {
                            postfix_tokens.push(
                                op_stack
                                    .pop()
                                    .expect("tried to pop operator off empty opStack"),
                            );
                        }
                        op_stack.push(token);
                    }
                }
            },
        }
    }

    while !op_stack.is_empty()
    {
        postfix_tokens.push(op_stack.pop().unwrap());
    }

    postfix_tokens
}

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

pub fn make_token(string: &str) -> Token
{
    match string
    {
        "+" => Token::Operator { op: Operation::Addition },
        "-" => Token::Operator { op: Operation::Subtraction },
        "*" => Token::Operator { op: Operation::Multiplication },
        "/" => Token::Operator { op: Operation::Division },
        "^" => Token::Operator { op: Operation::Exponentiation },

        "(" => Token::LSep,
        ")" => Token::RSep,

        number => Token::Number { val: number.to_string() },
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_tokenizer()
    {
        let tokens = create_tokens!["4", "+", "26", "/", "(", "8", "-", "2", ")", "^", "4"];

        let test1 = tokenize("4   +  26  /    (8-   2)  ^   4");
        let test2 = tokenize("4+ 26/ (8- 2)^  4");

        assert_eq!(test1, tokens);
        assert_eq!(test2, tokens);

        let tokens = create_tokens![
            "4", "(", "5", "-", "2", ")", "^", "(", "3", "*", "(", "5", "-", "6", ")", ")"
        ];
        let test1 = tokenize("4(5-2)^(3*(5-6))");
        let test2 = tokenize("4 (5-   2   )^(   3 *(    5-  6)  )");

        assert_eq!(test1, tokens);
        assert_eq!(test2, tokens);

        let tokens = create_tokens!["0004.", "+", ".23"];
        let test1 = tokenize("0004. +   .23");
        let test2 = tokenize("0004.+.23");

        assert_eq!(test1, tokens);
        assert_eq!(test2, tokens);

        let tokens = create_tokens![
            "0004", "^", "(", "0000.5", "/", "(", "3", "-", ".1", ")", "+", "2", ")", "-", ".2",
            "^", ".13", "-", ".23", "+", "23.22"
        ];
        let test1 = tokenize("0004^(0000.5/(3-.1)+2)-.2^.13-.23+23.22");
        let test2 = tokenize("0004^   (  0000.5/(3 -.1)    +2)-   .2 ^ .13 -.23+23.22");

        assert_eq!(test1, tokens);
        assert_eq!(test2, tokens);
    }

    #[test]
    fn test_infix_to_postfix()
    {
        let infix_tokens = create_tokens!["5"];
        let postfix_tokens = create_tokens!["5"];
        assert_eq!(shunting_yard(infix_tokens), postfix_tokens);

        let infix_tokens = create_tokens!["5", "+", "3"];
        let postfix_tokens = create_tokens!["5", "3", "+"];
        assert_eq!(shunting_yard(infix_tokens), postfix_tokens);

        let infix_tokens = create_tokens!["2", "*", "4", "+", "6"];
        let postfix_tokens = create_tokens!["2", "4", "*", "6", "+"];
        assert_eq!(shunting_yard(infix_tokens), postfix_tokens);

        let infix_tokens = create_tokens!["4", "*", "3", "+", "2", "^", "7"];
        let postfix_tokens = create_tokens!["4", "3", "*", "2", "7", "^", "+"];
        assert_eq!(shunting_yard(infix_tokens), postfix_tokens);

        let infix_tokens = create_tokens!["2", "^", "(", "3", "+", "4", ")"];
        let postfix_tokens = create_tokens!["2", "3", "4", "+", "^"];
        assert_eq!(shunting_yard(infix_tokens), postfix_tokens);
    }
}
