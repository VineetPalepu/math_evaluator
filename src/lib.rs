mod solver;
mod tokens;

use rand::{thread_rng, Rng};
use solver::*;
use tokens::*;

pub fn evaluate(expr: &str) -> f64
{
    let infix_tokens = tokenize(expr);
    let postfix_tokens = shunting_yard(infix_tokens);

    let expr_tree = ExpressionTree::from_postfix_tokens(postfix_tokens);

    expr_tree.eval()
}

pub fn generate_expression(terms: usize) -> String
{
    let (min, max) = (0f32, 10f32);

    let mut rng = thread_rng();
    let num1 = format!("{:.2}", rng.gen_range(min..max));

    let mut ops = Vec::new();
    let mut nums = Vec::new();

    fn get_rand_operator() -> String
    {
        let mut rng = thread_rng();
        let op = match rng.gen_range(0..=3)
        {
            0 => "+",
            1 => "-",
            2 => "*",
            3 => "/",
            _ => panic!("rand num bounds error"),
        };

        op.to_string()
    }

    for i in 0..terms - 1
    {
        ops.push(get_rand_operator());
        nums.push(rng.gen_range(min..max));
    }

    let mut expression = Vec::new();
    expression.push(num1);

    for i in 0..terms - 1
    {
        expression.push(ops[i].to_string());
        expression.push(format!("{:.2}", nums[i]));
    }

    expression.into_iter().collect()
}
