fn main()
{
    //let expr = "0004^(0000.5/(3-.1)+2)-.2^.13-.23+23.22";
    let expr = "2 ^ (3 + 4)";
    //let expr = "4+ 26/ (8- 2)^  4";
    //let expr = "2 + 4";

    math_evaluator::evaluate(expr);
}
