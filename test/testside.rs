use std::convert::TryInto;

fn main()
{
    //println!("{}", side("EQ", 'o'));
    //println!("{}", side("Equal (Plus (Num 1) (Num 2)) (Var (Id \"x\"))", 'o'));
    println!("{}", side("Equal (Plus (Num 1) (Num 2)) (Var (Id \"o\"))", 'o'));
}

fn side(equation: &str, variable: char) -> String
{
    //String should be of the form Equals (ExpL) (ExpR) where the left is the ExpL and the right is the ExpR
    let identifier = format!("(Var (Id \"{}\"))", variable );
   
    //check format of string
    let mut iter = equation.split_whitespace();
    if iter.next() != Some("Equal")
    {
        return String::from("INVALID: not an equation");
    }
    let tuple1 : (u8,u8) = parentheses(equation);
    let lhs: String = substring(equation, tuple1.0, tuple1.1 + 1);
    let tuple2: (u8, u8) = parentheses(&substring(   
                                                    equation,
                                                    tuple1.1 + 1 ,
                                                    equation.len().try_into().unwrap()
                                                ));
    let rhs: String = substring(equation, tuple2.0 + tuple1.1 + 1, tuple2.1 + tuple1.1 + 2);
    
    /*This section may require some explanation. Please refer to the write up to better understand the bounds of these Strings
     * Briefly however, the idea is that the 2nd string starts at the 2nd index +1 and the 2nd number is exclusive so we add 1 to thius always
     * for rhs this is +2 because the string is starting after the final index of the previous portion of the string
     * thus 1 for this shift, and another +1 for the exclusive. 
     */
    //println!("indices of first set of parens: {:?}\nindices of second set of parens: {:?}",tuple1,tuple2);
    //println!("LHS: {}\nRHS: {}", lhs, rhs);
            
    if lhs.contains(&identifier) && rhs.contains(&identifier)
    {
        return String::from("INVALID: the equation has the desired variable on 2 sides, please change input as this is currently beyond the capabilities of this machine");
    }
    if lhs.contains(&identifier)
    {
        return String::from("left");
    }
    if rhs.contains(&identifier)
    {
        return String::from("right");
    }
    //now look at the 2 cases in the next parentheses
    
   return format!("INVALID: maybe your equation: \n\"{}\"\n doesnt contain the variable \"{}\"", equation, variable);
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

/*
 * SUBSTRING returns a string of length b-a and is inclusive on a and exclusive on b. 
 * it will error is b is smaller than a or the bounds are not checked. This is fine
 * operators on a string literal and returns a heap allocated string 
 */
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