use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::tokens::*;

#[derive(Debug, PartialEq)]
pub struct Expression
{
    pub operator: Token,
    pub operands: Vec<Rc<RefCell<Expression>>>,
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

impl Expression
{
    pub fn from_postfix_tokens(postfix_tokens: Vec<Token>) -> Expression
    {
        let mut val_stack: Vec<Expression> = Vec::new();

        for token in postfix_tokens
        {
            match token
            {
                #[rustfmt::skip]
                Token::Number { .. } => val_stack.push(Expression { operator: token, operands: Vec::new() }),
                Token::Operator { .. } =>
                {
                    let num2 = val_stack.pop().expect("tried to pop value off empty stack");
                    let num1 = val_stack.pop().expect("tried to pop value off empty stack");
                    let new_children = vec_nodes![num1, num2];

                    val_stack.push(Expression { operator: token, operands: new_children });
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
        if self.operands.is_empty()
        {
            self.print_token();
        }
        else
        {
            print!("( ");
            self.operands[0].borrow().print_expression();
            self.print_token();
            self.operands[1].borrow().print_expression();
            print!(") ");
        }
    }

    fn print_token(&self)
    {
        match &self.operator
        {
            Token::Number { val } => print!("{} ", val),
            Token::Operator { op } => print!("{} ", op.str()),
            _ => panic!(
                "unexpected token encountered while printing: {:?}",
                self.operator
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
        match &self.operator
        {
            Token::Number { val } => print!("{}", val),
            Token::Operator { op } => match op
            {
                Operation::Addition =>
                {
                    print!("{{");
                    self.operands[0].borrow().print_expression_latex();
                    print!("}}+{{");
                    self.operands[1].borrow().print_expression_latex();
                    print!("}}");
                },
                Operation::Subtraction =>
                {
                    print!("{{");
                    self.operands[0].borrow().print_expression_latex();
                    print!("}}-{{");
                    self.operands[1].borrow().print_expression_latex();
                    print!("}}");
                },
                Operation::Multiplication =>
                {
                    print!("{{");
                    self.operands[0].borrow().print_expression_latex();
                    print!("}}\\cdot{{");
                    self.operands[1].borrow().print_expression_latex();
                    print!("}}");
                },
                Operation::Division =>
                {
                    print!("\\frac{{");
                    self.operands[0].borrow().print_expression_latex();
                    print!("}}{{");
                    self.operands[1].borrow().print_expression_latex();
                    print!("}}");
                },
                Operation::Exponentiation =>
                {
                    print!("{{");
                    self.operands[0].borrow().print_expression_latex();
                    print!("}}^{{");
                    self.operands[1].borrow().print_expression_latex();
                    print!("}}");
                },
            },
            _ => panic!(
                "unexpected token encountered while printing: {:?}",
                self.operator
            ),
        }
    }

    pub fn eval(&self) -> f64
    {
        // let tree = Rc::new(RefCell::new(self));

        Self::eval_helper(self)
    }

    fn eval_helper(tree_node: &Expression) -> f64
    {
        if tree_node.operands.is_empty()
        {
            return tree_node
                .operator
                .get_number()
                .expect("error getting number from token");
        }

        let children = &tree_node.operands;

        let c1 = children[0].borrow();
        let r1 = Self::eval_helper(&c1);

        let c2 = children[1].borrow();
        let r2 = Self::eval_helper(&c2);

        let op = tree_node
            .operator
            .get_operator()
            .expect(&format!(
                "error getting operator from token: {:?}",
                &tree_node.operator
            ))
            .clone();

        Self::eval_binary_op(r1, op, r2)
    }

    fn eval_binary_op(val1: f64, op: Operation, val2: f64) -> f64
    {
        match op
        {
            Operation::Addition => val1 + val2,
            Operation::Subtraction => val1 - val2,
            Operation::Multiplication => val1 * val2,
            Operation::Division => val1 / val2,
            Operation::Exponentiation => val1.powf(val2),
        }
    }

    pub fn simplify(self) -> f64
    {
        let tree: Rc<RefCell<Expression>> = Rc::new(RefCell::new(self));
        //tree.borrow().print();

        while matches!(&tree.borrow().operator, Token::Operator { .. })
        {
            let node_to_eval = Self::find_node(tree.clone());
            Self::evaluate_node(node_to_eval);
            //tree.borrow().print();
        }

        let result = tree
            .borrow()
            .operator
            .get_number()
            .expect("error getting number from token");

        result
    }

    pub fn find_node(root: Rc<RefCell<Expression>>) -> Rc<RefCell<Expression>>
    {
        let mut node_queue: VecDeque<Rc<RefCell<Expression>>> = VecDeque::new();
        let mut selected_node: Rc<RefCell<Expression>> = root.clone();

        node_queue.push_back(root);
        while !node_queue.is_empty()
        {
            let current_node = node_queue.pop_front().unwrap();

            if let Token::Operator { .. } = current_node.borrow().operator
            {
                node_queue.push_back(current_node.borrow().operands[0].clone());
                node_queue.push_back(current_node.borrow().operands[1].clone());

                selected_node = current_node.clone();
            };
        }

        selected_node
    }

    pub fn evaluate_node(node: Rc<RefCell<Expression>>)
    {
        let mut node = node.borrow_mut();

        let val1 = node.operands[0].borrow().operator.get_number().unwrap();
        let val2 = node.operands[1].borrow().operator.get_number().unwrap();

        if let Token::Operator { op } = &node.operator
        {
            let val = Self::eval_binary_op(val1, op.clone(), val2).to_string();
            node.operator = Token::Number { val };
            node.operands.clear();
        }
        else
        {
            panic!("attempted to eval invalid token: {:?}", node.operator);
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
        type Tree = Expression;
        let expression_tree = Tree {
            operator: make_token("+"),
            operands: vec_nodes![
                Tree {
                    operator: make_token("*"),
                    operands: vec_nodes![
                        Tree {
                            operator: make_token("4"),
                            operands: vec_nodes![],
                        },
                        Tree {
                            operator: make_token("3"),
                            operands: vec_nodes![],
                        },
                    ],
                },
                Tree {
                    operator: make_token("^"),
                    operands: vec_nodes![
                        Tree {
                            operator: make_token("2"),
                            operands: vec_nodes![],
                        },
                        Tree {
                            operator: make_token("7"),
                            operands: vec_nodes![],
                        },
                    ],
                },
            ],
        };

        assert_eq!(
            Expression::from_postfix_tokens(postfix_tokens),
            expression_tree
        );
    }

    #[test]
    fn test_evaluate_node()
    {
        type Tree = Expression;
        let expression_tree = Tree {
            operator: make_token("+"),
            operands: vec_nodes![
                Tree {
                    operator: make_token("5"),
                    operands: vec_nodes![],
                },
                Tree {
                    operator: make_token("3"),
                    operands: vec_nodes![],
                },
            ],
        };
        let result = Tree {
            operator: make_token("8"),
            operands: vec_nodes![],
        };

        let ptr = Rc::new(RefCell::new(expression_tree));
        Tree::evaluate_node(ptr.clone());
        assert_eq!(*ptr.borrow(), result);

        let tree1 = Tree {
            operator: make_token("+"),
            operands: vec_nodes![
                Tree {
                    operator: make_token("12"),
                    operands: vec_nodes![],
                },
                Tree {
                    operator: make_token("^"),
                    operands: vec_nodes![
                        Tree {
                            operator: make_token("2"),
                            operands: vec_nodes![],
                        },
                        Tree {
                            operator: make_token("7"),
                            operands: vec_nodes![],
                        },
                    ],
                },
            ],
        };
        let tree2 = Tree {
            operator: make_token("+"),
            operands: vec_nodes![
                Tree {
                    operator: make_token("*"),
                    operands: vec_nodes![
                        Tree {
                            operator: make_token("4"),
                            operands: vec_nodes![],
                        },
                        Tree {
                            operator: make_token("3"),
                            operands: vec_nodes![],
                        },
                    ],
                },
                Tree {
                    operator: make_token("^"),
                    operands: vec_nodes![
                        Tree {
                            operator: make_token("2"),
                            operands: vec_nodes![],
                        },
                        Tree {
                            operator: make_token("7"),
                            operands: vec_nodes![],
                        },
                    ],
                },
            ],
        };

        Tree::evaluate_node(tree2.operands[0].clone());

        assert_eq!(tree1, tree2);
    }

    #[test]
    fn test_find_node()
    {
        type Tree = Expression;

        let expression_tree = Tree {
            operator: make_token("+"),
            operands: vec_nodes![
                Tree {
                    operator: make_token("*"),
                    operands: vec_nodes![
                        Tree {
                            operator: make_token("4"),
                            operands: vec_nodes![],
                        },
                        Tree {
                            operator: make_token("3"),
                            operands: vec_nodes![],
                        },
                    ],
                },
                Tree {
                    operator: make_token("^"),
                    operands: vec_nodes![
                        Tree {
                            operator: make_token("2"),
                            operands: vec_nodes![],
                        },
                        Tree {
                            operator: make_token("7"),
                            operands: vec_nodes![],
                        },
                    ],
                },
            ],
        };

        let root = Rc::new(RefCell::new(expression_tree));
        assert_eq!(Tree::find_node(root.clone()), root.borrow().operands[1]);
    }
}
