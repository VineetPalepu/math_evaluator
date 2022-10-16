use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::str::FromStr;

use crate::tokens::*;

#[derive(Debug, PartialEq)]
pub struct ExpressionTree
{
    pub token: Token,
    pub children: Vec<Rc<RefCell<ExpressionTree>>>,
}

#[macro_export]
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
    pub fn from_postfix_tokens(postfix_tokens: Vec<Token>) -> ExpressionTree
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
            self.children[0].borrow().print_expression();
            self.print_token();
            self.children[1].borrow().print_expression();
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

    fn print_latex(&self)
    {
        //print!("\\[");
        self.print_expression_latex();
        //print!("\\]\\\\");
        println!();
    }

    fn print_expression_latex(&self)
    {
        match &self.token
        {
            Token::Number { val } => print!("{}", val),
            Token::Operator { op } => match op
            {
                Operation::Addition =>
                {
                    print!("{{");
                    self.children[0].borrow().print_expression_latex();
                    print!("}}+{{");
                    self.children[1].borrow().print_expression_latex();
                    print!("}}");
                },
                Operation::Subtraction =>
                {
                    print!("{{");
                    self.children[0].borrow().print_expression_latex();
                    print!("}}-{{");
                    self.children[1].borrow().print_expression_latex();
                    print!("}}");
                },
                Operation::Multiplication =>
                {
                    print!("{{");
                    self.children[0].borrow().print_expression_latex();
                    print!("}}\\cdot{{");
                    self.children[1].borrow().print_expression_latex();
                    print!("}}");
                },
                Operation::Division =>
                {
                    print!("\\frac{{");
                    self.children[0].borrow().print_expression_latex();
                    print!("}}{{");
                    self.children[1].borrow().print_expression_latex();
                    print!("}}");
                },
                Operation::Exponentiation =>
                {
                    print!("{{");
                    self.children[0].borrow().print_expression_latex();
                    print!("}}^{{");
                    self.children[1].borrow().print_expression_latex();
                    print!("}}");
                },
            },
            _ => panic!(
                "unexpected token encountered while printing: {:?}",
                self.token
            ),
        }
    }

    pub fn eval(self) -> f64
    {
        let tree: Rc<RefCell<ExpressionTree>> = Rc::new(RefCell::new(self));
        tree.borrow().print();

        while matches!(&tree.borrow().token, Token::Operator { .. })
        {
            let node_to_eval = Self::find_node(tree.clone());
            Self::evaluate_node(node_to_eval);
            tree.borrow().print();
        }

        let result = tree
            .borrow()
            .token
            .get_number()
            .expect("token was no number")
            .clone();

        f64::from_str(&result).expect(&format!("error converting token data to number: {result}"))
    }

    pub fn find_node(root: Rc<RefCell<ExpressionTree>>) -> Rc<RefCell<ExpressionTree>>
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

    pub fn evaluate_node(node1: Rc<RefCell<ExpressionTree>>)
    {
        let mut node = node1.borrow_mut();
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
    use std::{cell::RefCell, rc::Rc};

    use crate::create_tokens;

    use super::*;

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
