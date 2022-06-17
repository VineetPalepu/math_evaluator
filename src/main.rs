use std::env;

fn main()
{
    let args: Vec<String> = env::args().collect();
    let expr = &args[1];
    println!("{}", expr);

    math_evaluator::evaluate(expr);
    /*
    //let expr = "0004^(0000.5/(3-.1)+2)-.2^.13-.23+23.22";
    //let expr = "2 ^ (3 + 4)";
    //let expr = "4+ 26/ (8- 2)^  4";
    let expr = "2 + 4 + 6 + 8 + 10";
    //let expr = "(4*2-(8-3))/(5^.4*2) + 5^(.2-.8/3)";

    math_evaluator::evaluate(expr);
    */
}
