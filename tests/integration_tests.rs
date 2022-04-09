use math_evaluator::evaluate;

#[test]
fn test_eval()
{
    assert_eq!(evaluate("2+2"), 4.);
}