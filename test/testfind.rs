use std::convert::TryInto;

fn main()
{
    assert_eq!(-1, find("Times (Plus (Num 1) (Var (Id \"a\"))) (Var (Id \"a\"))", 'a')); //exists in both parts
    assert_eq!(-2, find("Times (Plus (Num 1) (Var (Id \"a\"))) (Var (Id \"a\"))", 'b')); //doesnt exist in either
    assert_eq!(1, find("Plus (Var (Id \"f\")) (Dbl 2)", 'f')); //exist in left block
    assert_eq!(2, find("Plus (Int 2) (Var (Id \"f\"))", 'f')); //exist in right block
    assert_eq!(2, find("Equal (Plus (Num 1) (Num 2)) (Var (Id \"o\"))", 'o'));
}

/*
 * FIND will give the expression that the desired value is a part of. 
 * This will allow for the function to keep performing the apply and eventually solve the equation for the new variable
 * will return 1 for the first argument and 2 for the second
 */
 fn find(string: &str, identifier: char) -> i8
 {
     let variable = format!("(Var (Id \"{}\"))", identifier );
     let expressions = twoparameters(string);
     //let expressions = twoparameters(string);
     if expressions.0.contains(&variable) && expressions.1.contains(&variable)
     {
         return -1; // we can not solve these types of equations at this stage, 
                    // this could be a future development, or also, just rewrite the equation as an approximation 
     }
     if expressions.0.contains(&variable)
     {
         return 1;
     }
     if expressions.1.contains(&variable)
     {
         return 2;
     }
     return -2; // doesnt exist in the equation, is this possible? 
                // theorehtically but witht the conditions on the inputs we have check to make sure it is valid
 }
 

 fn twoparameters(parameter: &str) -> (String, String)
{
    let first: (u8, u8) =  parentheses(parameter);
    let ending = substring(parameter, (first.1+1).try_into().unwrap(), parameter.len().try_into().unwrap());
    let second: (u8, u8) = parentheses(&ending);
    return (substring(parameter, first.0.try_into().unwrap(), (first.1+1).try_into().unwrap()),
            substring(&ending, second.0.try_into().unwrap(), (second.1+1).try_into().unwrap()));
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