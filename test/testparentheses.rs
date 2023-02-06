use std::convert::TryInto;

fn main()
{
    println!("{:?}", parentheses("0123(5(7)9)")); //4,10
}
/*
 * PARENTHESES returns the index of the first opening parentheses and its corresponding closing parenthesis
 * the function is unsafe as there is a component using unsafe keyword, 
 * this only means that input must be UTF8 for the code to be safe.
 * This function also requires correct input which is why we are using the input from the grammar to 
 * perform our algebra
 */
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