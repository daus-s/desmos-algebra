use std::convert::TryInto;

fn main()
{
    println!("{:#?}", all_variables("Equal (Var (Id \"V\")) (Times (Var (Id \"I\")) (Var (Id \"R\")))"));
}


fn all_variables(equation: &str) -> Vec<char>
{   
    let mut variables : Vec<char> = Vec::new();
    //      012345678901234
    //Equal (Var (Id "V")) (Times (Var (Id "I")) (Var (Id "R")))"
    let prefix = "(Var (Id \"";
    let suffix = "\"))";
    let mut max_index = equation.len().try_into().unwrap();
    max_index -= 14;
    for i in 0..max_index
    {
        let preslice = substring(equation, i, i+10);
        let postslice = substring(equation, i+11, i+14);

        if prefix==preslice && suffix==postslice
        {
            variables.push(equation.chars().nth((i+10).into()).unwrap());
        }
    }
    variables    
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