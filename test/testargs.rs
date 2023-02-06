use std::convert::TryInto;

fn main()
{
    let values: (String, String) = args("Equal (Plus (Num 1) (Num 2)) (Var (Id \"o\"))");
    println!("Exp: {}\nParameters: {}", values.0, values.1);
    assert_eq!(("Int".to_string(), "1".to_string()), dbg!(args("Int 1")));
    dbg!(args("Pi"));
    dbg!(args("Var (ID \"a\")"));
    dbg!(args("Id \"a\""));

}
/*
 * ARGS will return the commmand type, and the following arguments or parameters of the function
 * as a single string. For the way to use these as information look at the main function in testargs.rs
 */
fn args(input: &str) -> (String, String)
{
    //String will have the structure of an expression with 1 or 2 parameters
    let mut iter = input.split_whitespace();
    let expression: &str = iter.next().unwrap();
    if expression == "Int" || expression == "Dbl"
    {
        return (String::from(expression), String::from(iter.next().unwrap()));
    }
    if expression == "Pi" || expression == "E"
    {
        return (String::from(expression), String::from(""));
    }
    if expression == "Id"
    {
        return (String::from("Id"), substring(iter.next().unwrap(), 1, 2));
    }


    //println!("{}", expression);
    let l: u8 = expression.len().try_into().unwrap();
    let parameter_string: String = substring(input, l, input.len().try_into().unwrap());
    let binding = parameter_string.to_string();
    let parameter = binding.trim();    
    //println!("{}", parameter);
    return (String::from(expression),
            String::from(parameter));
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