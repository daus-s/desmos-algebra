fn main()
{
    assert_eq!("Minus", invert("Plus"));
    assert_eq!("Plus", invert("Minus"));
    assert_eq!("Div", invert("Times"));
    assert_eq!("Times", invert("Div"));
    assert_eq!("Negate", invert("Negate"));
    assert_eq!("Exp2", invert("Sqrt"));
    assert_eq!("Exp10", invert("Log"));
    assert_eq!("Ln", invert("ExpE"));
    assert_eq!("NRt", invert("ExpN"));
    assert_eq!("Sin", invert("ASin"));
    assert_eq!("ASin", invert("Sin"));
    assert_eq!("Cos", invert("ACos"));
    assert_eq!("ACos", invert("Cos"));
    assert_eq!("Tan", invert("ATan"));
    assert_eq!("ATan", invert("Tan"));
    assert_eq!("NULL", invert("askdbjasjhbdashbjd"));

}
/*
 * INVERT takes a string of some expression that is supported by the interpreter and the rewrite system this and returns the string of the inverse function 
 * e.g. invert (divide a b) -> (times b *) where the star is the side opposite, however this just returns the part that is the inverse function
 * the theory behind all of this is described in depth in the paper included with the report  
 * This is essentially a table to return the inverse function mathematically. It will not return the arguments of the expression. 
 * this portion of the problem will be solved by a nother component
 * this value must be processed at another point to continue colving the input equation
 */
fn invert(arg: &str) -> &str 
{
    if arg == "Plus"
    {
        return "Minus";
    }
    if arg == "Minus"
    {
        return "Plus";
    }
    if arg == "Times"
    {
        return "Div";
    }
    if arg == "Div"
    {
        return "Times"  ;
    }
    if arg == "Negate"
    {
        return "Negate";
    }
     /* Factorial does not have an inverse function (it does we may support it in the future)
     * thus some functions supported unto this point will not be solvable by this program
     */
    if arg == "Exp" //https://www.rapidtables.com/math/algebra/logarithm/Logarithm_Base_Change.html
    {
        return "Log";
    }
    if arg == "Log"
    {
        return "Exp10" ;
    }
    if arg=="Ln"
    {
        return "ExpE";
    }
    if arg == "Sqrt"
    {
        return "Exp2";
    }
    if arg=="NRt"
    {
        return "ExpN";
    }
    if arg == "Sin"
    {
        return "ASin";
    }
    if arg == "ASin"
    {
        return "Sin";
    }
    if arg == "Cos"
    {        
        return "ACos";
    }
    if arg == "ACos"
    {
        return "Cos";
    }
    if arg == "Tan"
    {
        return "ATan";
    }
    if arg == "ATan"
    {
        return "Tan" ;
    }
    return "NULL";

    //no inverse for round exists, a proof is possible in 1 line
    //round 1.4 = 1, round 1.3 = 1, inv_round 1 = 1.3, inv_round = 1.4 .: inv_round is not a function
}