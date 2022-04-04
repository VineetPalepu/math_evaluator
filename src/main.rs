use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::str::FromStr;

fn main()
{
    let expr = "0004^(0000.5/(3-.1)+2)-.2^.13-.23+23.22";
    //let expr = "4+ 26/ (8- 2)^  4";
    //let expr = "2 + 4";
    ExpressionTree::from_str(expr).eval();
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

fn infix_to_postfix(infix_tokens: Vec<Token>) -> Vec<Token>
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
                            && (cur_op.precedence()
                                < match op_stack.last().unwrap()
                                {
                                    Token::Operator { op } => op,
                                    tok => panic!("token {:?} is not an operator", tok),
                                }
                                .precedence()
                                || (cur_op.precedence()
                                    == match op_stack.last().unwrap()
                                    {
                                        Token::Operator { op } => op,
                                        tok => panic!("token {:?} is not an operator", tok),
                                    }
                                    .precedence()
                                    && cur_op.associativity() == Associativity::Left))
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

impl Token
{
    fn get_number(&self) -> Option<&String>
    {
        match self
        {
            Token::Number { val } => Some(val),
            _ => None,
        }
    }

    fn get_operator(&self) -> Option<&Operation>
    {
        match self
        {
            Token::Operator { op } => Some(op),
            _ => None,
        }
    }
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
            #[rustfmt::skip]
            Self::Addition | Self::Subtraction | Self::Multiplication | Self::Division => Associativity::Left,
            Self::Exponentiation => Associativity::Right,
        }
    }
}

#[derive(Debug, PartialEq)]
struct ExpressionTree
{
    token: Token,
    children: Vec<Rc<RefCell<ExpressionTree>>>,
}

macro_rules! vec_nodes {
    ($($i:ident),*) => {
        vec![
        $(
            Rc::new(RefCell::new($i)),
        )*
        ]
    };
    ($($i:expr),*) => {
        vec![
        $(
            Rc::new(RefCell::new($i)),
        )*
        ]
    };
    ($($i:expr,)*) => {
        vec![
        $(
            Rc::new(RefCell::new($i)),
        )*
        ]
    };
}

impl ExpressionTree
{
    fn from_str(string: &str) -> ExpressionTree
    {
        Self::from_postfix_tokens(infix_to_postfix(tokenize(string)))
    }

    fn from_postfix_tokens(postfix_tokens: Vec<Token>) -> ExpressionTree
    {
        let mut val_stack: Vec<ExpressionTree> = Vec::new();

        for token in postfix_tokens
        {
            match token
            {
                #[rustfmt::skip]
                Token::Number { .. } => val_stack.push(ExpressionTree { token, children: Vec::new() }),
                Token::Operator { .. } =>
                {
                    let num2 = val_stack.pop().expect("tried to pop value off empty stack");
                    let num1 = val_stack.pop().expect("tried to pop value off empty stack");
                    let new_children = vec_nodes![num1, num2];

                    val_stack.push(ExpressionTree { token, children: new_children });
                },
                Token::LSep | Token::RSep =>
                {
                    panic!("unexpected token for postfix expression: {:?}", token)
                },
            }
        }

        assert!(val_stack.len() == 1);
        val_stack.pop().unwrap()
    }

    fn print(&self)
    {
        self.print_expression();
        println!();
    }

    fn print_expression(&self)
    {
        if self.children.is_empty()
        {
            self.print_token();
        }
        else
        {
            print!("( ");
            (*self.children[0]).borrow().print_expression();
            self.print_token();
            (*self.children[1]).borrow().print_expression();
            print!(") ");
        }
    }

    fn print_token(&self)
    {
        match &self.token
        {
            Token::Number { val } => print!("{} ", val),
            Token::Operator { op } => print!("{} ", op.str()),
            _ => panic!(
                "unexpected token encountered while printing: {:?}",
                self.token
            ),
        }
    }

    fn eval(self)
    {
        let tree: Rc<RefCell<ExpressionTree>> = Rc::new(RefCell::new(self));
        tree.borrow().print();

        while let Token::Operator { .. } = tree.borrow().token
        {
            Self::evaluate_node(Self::find_node(tree.clone()));
            tree.borrow().print();
        }
    }

    fn find_node(root: Rc<RefCell<ExpressionTree>>) -> Rc<RefCell<ExpressionTree>>
    {
        let mut node_queue: VecDeque<Rc<RefCell<ExpressionTree>>> = VecDeque::new();
        let mut selected_node: Rc<RefCell<ExpressionTree>> = root.clone();

        node_queue.push_back(root);
        while !node_queue.is_empty()
        {
            let current_node = node_queue.pop_front().unwrap();

            if let Token::Operator { .. } = current_node.borrow().token
            {
                node_queue.push_back(current_node.borrow().children[0].clone());
                node_queue.push_back(current_node.borrow().children[1].clone());

                selected_node = current_node.clone();
            };
        }

        selected_node
    }

    fn evaluate_node(node: Rc<RefCell<ExpressionTree>>)
    {
        let mut node = (*node).borrow_mut();
        match &node.token
        {
            Token::Operator { op } => match op
            {
                Operation::Addition =>
                {
                    let val1 = f64::from_str(node.children[0].borrow().token.get_number().unwrap())
                        .unwrap();
                    let val2 = f64::from_str(node.children[1].borrow().token.get_number().unwrap())
                        .unwrap();

                    node.token = Token::Number { val: (val1 + val2).to_string() };
                    node.children.clear();
                },
                Operation::Subtraction =>
                {
                    let val1 = f64::from_str(node.children[0].borrow().token.get_number().unwrap())
                        .unwrap();
                    let val2 = f64::from_str(node.children[1].borrow().token.get_number().unwrap())
                        .unwrap();

                    node.token = Token::Number { val: (val1 - val2).to_string() };
                    node.children.clear();
                },
                Operation::Multiplication =>
                {
                    let val1 = f64::from_str(node.children[0].borrow().token.get_number().unwrap())
                        .unwrap();
                    let val2 = f64::from_str(node.children[1].borrow().token.get_number().unwrap())
                        .unwrap();

                    node.token = Token::Number { val: (val1 * val2).to_string() };
                    node.children.clear();
                },
                Operation::Division =>
                {
                    let val1 = f64::from_str(node.children[0].borrow().token.get_number().unwrap())
                        .unwrap();
                    let val2 = f64::from_str(node.children[1].borrow().token.get_number().unwrap())
                        .unwrap();

                    node.token = Token::Number { val: (val1 / val2).to_string() };
                    node.children.clear();
                },
                Operation::Exponentiation =>
                {
                    let val1 = f64::from_str(node.children[0].borrow().token.get_number().unwrap())
                        .unwrap();
                    let val2 = f64::from_str(node.children[1].borrow().token.get_number().unwrap())
                        .unwrap();

                    node.token = Token::Number {
                        val: (val1.powf(val2)).to_string(),
                    };
                    node.children.clear();
                },
            },
            _ => panic!("attempted to eval invalid token: {:?}", node.token),
        }
    }
}

#[cfg(test)]
mod tests
{
    use std::vec;

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

    fn make_token(string: &str) -> Token
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

        let tokens = create_tokens!["0004", "^", "(", "0000.5", "/", "(", "3", "-", ".1", ")", "+", "2", ")", "-", ".2", "^", ".13", "-", ".23", "+", "23.22"];
        let test1 = tokenize("0004^(0000.5/(3-.1)+2)-.2^.13-.23+23.22");
        let test2 = tokenize("0004^   (  0000.5/(3 -.1)    +2)-   .2 ^ .13 -.23+23.22");
    }

    #[test]
    fn test_infix_to_postfix()
    {
        let infix_tokens = create_tokens!["5"];
        let postfix_tokens = create_tokens!["5"];
        assert_eq!(infix_to_postfix(infix_tokens), postfix_tokens);

        let infix_tokens = create_tokens!["5", "+", "3"];
        let postfix_tokens = create_tokens!["5", "3", "+"];
        assert_eq!(infix_to_postfix(infix_tokens), postfix_tokens);

        let infix_tokens = create_tokens!["4", "*", "3", "+", "2", "^", "7"];
        let postfix_tokens = create_tokens!["4", "3", "*", "2", "7", "^", "+"];
        assert_eq!(infix_to_postfix(infix_tokens), postfix_tokens);

        let infix_tokens = create_tokens!["0004", "^", "(", "0000.5", "/", "(", "3", "-", ".1", ")", "+", "2", ")", "-", ".2", "^", ".13", "-", ".23", "+", "23.22"];
        let postfix_tokens = create_tokens![""]; // TODO
        assert_eq!(infix_to_postfix(infix_tokens), postfix_tokens);
    }

    #[test]
    fn test_tree_from_postfix()
    {
        let postfix_tokens = create_tokens!["4", "3", "*", "2", "7", "^", "+"];
        type Tree = ExpressionTree;
        let expression_tree = Tree {
            token: make_token("+"),
            children: vec_nodes![
                Tree {
                    token: make_token("*"),
                    children: vec_nodes![
                        Tree {
                            token: make_token("4"),
                            children: vec_nodes![],
                        },
                        Tree {
                            token: make_token("3"),
                            children: vec_nodes![],
                        },
                    ],
                },
                Tree {
                    token: make_token("^"),
                    children: vec_nodes![
                        Tree {
                            token: make_token("2"),
                            children: vec_nodes![],
                        },
                        Tree {
                            token: make_token("7"),
                            children: vec_nodes![],
                        },
                    ],
                },
            ],
        };

        assert_eq!(
            ExpressionTree::from_postfix_tokens(postfix_tokens),
            expression_tree
        );
    }

    #[test]
    fn test_evaluate_node()
    {
        type Tree = ExpressionTree;
        let expression_tree = Tree {
            token: make_token("+"),
            children: vec_nodes![
                Tree {
                    token: make_token("5"),
                    children: vec_nodes![],
                },
                Tree {
                    token: make_token("3"),
                    children: vec_nodes![],
                },
            ],
        };
        let result = Tree {
            token: make_token("8"),
            children: vec_nodes![],
        };

        let ptr = Rc::new(RefCell::new(expression_tree));
        Tree::evaluate_node(ptr.clone());
        assert_eq!(*ptr.borrow(), result);

        let tree1 = Tree {
            token: make_token("+"),
            children: vec_nodes![
                Tree {
                    token: make_token("12"),
                    children: vec_nodes![],
                },
                Tree {
                    token: make_token("^"),
                    children: vec_nodes![
                        Tree {
                            token: make_token("2"),
                            children: vec_nodes![],
                        },
                        Tree {
                            token: make_token("7"),
                            children: vec_nodes![],
                        },
                    ],
                },
            ],
        };
        let tree2 = Tree {
            token: make_token("+"),
            children: vec_nodes![
                Tree {
                    token: make_token("*"),
                    children: vec_nodes![
                        Tree {
                            token: make_token("4"),
                            children: vec_nodes![],
                        },
                        Tree {
                            token: make_token("3"),
                            children: vec_nodes![],
                        },
                    ],
                },
                Tree {
                    token: make_token("^"),
                    children: vec_nodes![
                        Tree {
                            token: make_token("2"),
                            children: vec_nodes![],
                        },
                        Tree {
                            token: make_token("7"),
                            children: vec_nodes![],
                        },
                    ],
                },
            ],
        };

        Tree::evaluate_node(tree2.children[0].clone());

        assert_eq!(tree1, tree2);
    }

    #[test]
    fn test_find_node()
    {
        type Tree = ExpressionTree;

        let expression_tree = Tree {
            token: make_token("+"),
            children: vec_nodes![
                Tree {
                    token: make_token("*"),
                    children: vec_nodes![
                        Tree {
                            token: make_token("4"),
                            children: vec_nodes![],
                        },
                        Tree {
                            token: make_token("3"),
                            children: vec_nodes![],
                        },
                    ],
                },
                Tree {
                    token: make_token("^"),
                    children: vec_nodes![
                        Tree {
                            token: make_token("2"),
                            children: vec_nodes![],
                        },
                        Tree {
                            token: make_token("7"),
                            children: vec_nodes![],
                        },
                    ],
                },
            ],
        };

        let root = Rc::new(RefCell::new(expression_tree));
        assert_eq!(Tree::find_node(root.clone()), root.borrow().children[1]);
    }
}
