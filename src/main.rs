use std::env;

use math_evaluator::generate_expression;

fn main()
{
    // MAIN
    let args: Vec<String> = env::args().collect();
    let expr = if args.len() == 2
    {
        args[1].clone()
    }
    else
    {
        math_evaluator::generate_expression(1000)
    };

    println!("{}", expr);

    math_evaluator::evaluate(&expr);

    // TESTING
    /*
    // let expr = "0004^(0000.5/(3-.1)+2)-.2^.13-.23+23.22";
    // let expr = "2 ^ (3 + 4)";
    // let expr = "4+ 26/ (8- 2)^  4";
    // let expr = "2 + 4 + 6 + 8 + 10";
    // let expr = "(4*2-(8-3))/(5^.4*2) + 5^(.2-.8/3)";
    math_evaluator::evaluate(&expr);
    */
}

// TODO: implement
//
// multi-threaded solving of large expressions (on gpu?)
//      find multiple leaf nodes at once and solve simultaneously
//      or maybe find N (where N is the number of cores/threads) nodes in the expression tree where none are childen of each other and
//      solve the subtrees independently
//      if translating to gpu can maybe queue up operations and operands then push to gpu and evaluate
//
//
// Formal syntax specification:
//
//
// expression = number (bin_op number)?
// expression = "(" expression ")"
// number = "-"? digit+ ("."digit+)?
// bin_op = "+" | "-" | "*" | "/"
