mod solver;
mod tokens;

use solver::*;
use tokens::*;

pub fn evaluate(expr: &str) -> f64
{
    let infix_tokens = tokenize(expr);
    let postfix_tokens = shunting_yard(infix_tokens);

    let expr_tree = ExpressionTree::from_postfix_tokens(postfix_tokens);

    expr_tree.eval()
}
