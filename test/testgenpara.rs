use std::convert::TryInto;

fn main()
{
    println!("{}", oneparameter("Negate (Num 6)"));
    let values = twoparameters("Equal (Plus (Num 1) (Num 2)) (Var (Id \"o\"))");
    println!("first: {}\nsecond: {}", values.0, values.1);
    

}

fn oneparameter(parameter: &str) -> String
{
    let first: (u8, u8) =  parentheses(parameter);
    return substring(parameter, first.0.try_into().unwrap(), (first.1+1).try_into().unwrap());
}
fn twoparameters(parameter: &str) -> (String, String)
{
    let first: (u8, u8) =  parentheses(parameter);
    let ending = substring(parameter, (first.1+1).try_into().unwrap(), parameter.len().try_into().unwrap());
    let second: (u8, u8) = parentheses(&ending);
    return (substring(parameter, first.0.try_into().unwrap(), (first.1+1).try_into().unwrap()),
            substring(&ending, second.0.try_into().unwrap(), (second.1+1).try_into().unwrap()));
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