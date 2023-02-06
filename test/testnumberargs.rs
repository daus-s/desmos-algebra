use std::convert::TryInto;

fn main()
{
    assert_eq!(2, numberargs("(Plus (Num 1) (Num 2)) (Var (Id \"o\"))"));
    assert_eq!(1, numberargs("(Sqrt (Num 16))"));
    assert_eq!(1, numberargs("(Num 16)"));
    assert_eq!(1, numberargs("Var (Id \"V\")"));
}
/*
 * NUMBERARGS returns the number of arguments that a expression in the
 * grammar has. This includes the basic operators as well as supported functions
 * The parameters must be surrounded by parenthese but this is what the grammar
 * creates so this is good behavior
 */
fn numberargs(parameters: &str) -> u8
{
    /* parameter is the string of arguments that the expression takes for example
     * Plus -> parameter:  "(Exp e) (Exp f)" 
     * however some expressions take only 1 argument.
     * Negate -> parameter: "(Exp e)"
     * why does this help us? because we would like to know how many parameters we need to parse
     * and later rewrite the equation with
     */
     //(Plus (Num 1) (Num 2)) (Var (Id "o"))
    let first: (u8, u8) =  parentheses(parameters);
    

    let ending = substring(parameters, (first.1+1).try_into().unwrap(), parameters.len().try_into().unwrap());
    let second: (u8, u8) = parentheses(&ending);
    if second.0 == second.1
    {
        1
    }
    else 
    {
        2
    }
}

fn parentheses(string: &str) -> (u8, u8)
{
    let s = String::from(string);
    let mut index: u8 = 0;
    let mut depth: u8 = 0;
    let mut first: bool = true;
    let mut initial_index: u8 = 0;
    let mut final_index: u8 = string.len().try_into().unwrap();

   
    let vec = s.as_bytes();
    for i in vec {
        // iterate by-value
        let j: u8 = *i; // elements are values
        if j == 40 
        {
            if first
            {
                first = false;
                initial_index = index;
            }
            depth += 1;
        }
        if j == 41
        {
            depth -= 1;
            if depth == 0
            {
                final_index = index;
                break;
            }
        }
        index+=1;
    }
    (initial_index,final_index)
}
fn substring(original: &str, a: u8, b: u8) -> String
{
    //patient in the sense that it is being operated on
    let mut patient = String::from(original);
    let mut selection = patient.split_off(a.into());
    //selection now has the part we want but has the end to be discarded as well
    let d: u8 = b - a;
    let _trailing = selection.split_off(d.into());
    return selection;
}