use std::convert::TryInto;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::Command;


// This is the main function
fn main() 
{
    /* 
    println!("{}", solve("Equal (Var (Id \"x\")) (Int 1)", 'x'));
    println!("{}", solve("Equal (Plus (Int 2) (Var (Id \"x\"))) (Int 1)", 'x'));
    println!("{}", solve("Equal (Times (Int 2) (Var (Id \"x\"))) (Plus (Int 1) (Dbl 4.56))", 'x')); 
    println!("{}", solve("Equal (Var (Id \"V\")) (Times (Var (Id \"I\")) (Var (Id \"R\")))", 'I')); //ohms law
    println!("{}", solve("Equal (Var (Id \"V\")) (Times (Var (Id \"I\")) (Var (Id \"R\")))", 'R')); //ohms law
    println!("{}", solve("Equal (Var (Id \"V\")) (Times (Var (Id \"I\")) (Var (Id \"R\")))", 'V')); //ohms law
    


    println!("{}", translate("(Dbl 3.14)"));
    println!("{}", translate("Equal (Dbl 3.14) (Pi)"));*/
    //printmany(all_equations("Equal (Dbl 56.5) (Times (Var (Id \"a\")) (Plus (E) (Int 837)))"));


    let mut equation: String = String::from("");
    if let Ok(lines) = read_lines("./iasd1844.desmos") 
    {
        // Consumes the iterator, returns an (Optional) String
        for line in lines 
        {
            if let Ok(eqn) = line 
            {
                //replace Pi with (Pi) and E with (E) we have to for formating
                equation = formatepi(&eqn);
            }
        }
    }
    printmany(all_equations(&equation));
}


//Rust by example code
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


/*
 * FORMATEPI returns the original string wiht parentheses on pi and e to deal with one and two parameters
 */
fn formatepi(original: &str) -> String
{
    return original.replace(" Pi ", " (Pi) ").replace(" E ", " (E) ");
}

/*
 * REMOVEQUOTES returns the original stirng without quotes 
 */
fn remove_quotes(original: &str) -> String
{
    String::from(original.replace("\"", ""))
}

/*
 * PRINT prints a single linearized expresssion
 */
fn print(eqn: &str)
{
    println!("{}", remove_quotes(&translate(eqn)));
}

/*
 * PRINTMANY prints all the expresssions in a vector linearized 
 */
fn printmany(list: Vec<String>) -> ()
{
    for eqn in list
    {
        println!("{}", remove_quotes(&translate(&eqn)));
    }
    return;
}

/*
 * ALLEQUATIONS returns every single equation solved for all variables in it
 */
fn all_equations(eqn: &str) -> Vec<String>
{
    let mut equations : Vec<String> = Vec::new();
    //sometimes this gives bad results from solve so we factor for that
    let variables = all_variables(eqn);
    for var in variables
    {
        //println!("var: {}\neqn: {}", var, eqn);
        if solve(eqn, var) != "BAD EQUATION"
        {
            equations.push(solve(eqn, var));
            //println!("{:#?}", equations);
        }
    }
    return equations;
}
/*
 * ALLVARIABLES returns a vector of all the variables used in the equation
 */
fn all_variables(equation: &str) -> Vec<char>
{   
    let mut variables : Vec<char> = Vec::new();
    //      012345678901234
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
/*
 * TRANSLATION_TABLE returns a hashmap with the key the expression name and the value (of the kvp)
 * the operator in mathematical syntax. 
 * 
 */
fn translation_table() -> HashMap<&'static str, &'static str>
{
    HashMap::from([
        ("Equal", "ARGUMENT1=ARGUMENT2"),
        ("Minus","ARGUMENT1-ARGUMENT2"),
        ("Plus","ARGUMENT1+ARGUMENT2"),
        ("Times","ARGUMENT1\\cdotARGUMENT2"),
        ("Div","\\frac{(ARGUMENT1)}{(ARGUMENT2)}"),
        ("Negate","-ARGUMENT"),
        ("Exp","ARGUMENT1^ARGUMENT2"),
        ("Fact","ARGUMENT!"),
        ("Sqrt","\\sqrt{(ARGUMENT)}"),
        ("NRt","\\sqrt[(ARGUMENT1)]{(ARGUMENT2)}"),
        ("Sin","\\sin\\left(ARGUMENT\\right)"),
        ("Cos","\\cos\\left(ARGUMENT\\right)"),
        ("Tan","\\tan\\left(ARGUMENT\\right)"),
        ("ASin","\\sin^{-1}\\left(ARGUMENT\\right)"),
        ("ACos","\\cos^{-1}\\left(ARGUMENT\\right)"),
        ("ATan","\\tan^{-1}\\left(ARGUMENT\\right)"),
        ("Ln","\\lnARGUMENT"),
        ("Log","\\logARGUMENT"),
        ("Abs","\\left|ARGUMENT\\right|"),
        ("E","e"),
        ("Pi","\\pi"),
        ("Int","ARGUMENT"),
        ("Dbl","ARGUMENT"),
        ("Var","ARGUMENT"),
        ("Id","ARGUMENT")])
}


/*
 * TRANSLATE returns a string that is in the linearized form and is mathematically legible
 */
fn translate(equation: &str) -> String
{
    let table = translation_table();

    let sentence = trimparentheses(&equation);
    let expression = table.get(&*args(&sentence).0).unwrap().to_string();
    let operator = args(&sentence).0;
    if operator == "Pi"
    {
        return String::from("\\pi");
    }
    if operator == "E"
    {
        return String::from("e");
    }
    if operator == "Id"
    {
        return trimparentheses(&args(&sentence).1);
    }
    if operator == "Int" || operator == "Dbl"
    {
        return args(&sentence).1;
    }
    
    let argument_count = numberargs(&sentence);
    if argument_count == 1
    {
        let parameter = oneparameter(&sentence);
        return expression.replace("ARGUMENT", &translate(&parameter));
    }
    if argument_count == 2
    {
        let parameters = twoparameters(&sentence);
        return expression.replace("ARGUMENT1", &translate(&parameters.0)).replace("ARGUMENT2", &translate(&parameters.1));
    }
    return String::from("INVALID: something went wrong");
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

/*
 * SIDE will return the side of the string in whichb the desired variable is a part of 
 * examples;;
 *      equation: y+x = z*c, variable: c -> "right"
 *      equation: 1+x^5 = 8, variable: x -> "left"
 */
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


 /*
  * PARENTHESES returns the index of the first opening parentheses and its corresponding closing parenthesis
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

/*
 * ARGS will return the commmand type, and the following arguments or parameters of the function
 * as a single string. For the way to use these as information look at the main function in testargs.rs
 */
 fn args(input: &str) -> (String, String)
 {
     //String will have the structure of an expression with 1 or 2 parameters
     let mut iter = input.split_whitespace();
     let expression: &str = iter.next().unwrap();
     //println!("{}", expression);
     let l: u8 = expression.len().try_into().unwrap();
     
     let u: u8 = input.len().try_into().unwrap();
     let parameter_string: String = substring(input, l, u);
     let binding = parameter_string.to_string();
     let parameter = binding.trim();    
     //println!("{}", parameter);
     return (String::from(expression),
             String::from(parameter));
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
/*
 * ONEPARAMETER returns the first argument of a string. 
 * the parentheses need to be trimmed formt he outermost expression but this is done by the grammar. 
 */
fn oneparameter(parameter: &str) -> String
{
    let first: (u8, u8) =  parentheses(parameter);
    return substring(parameter, first.0.try_into().unwrap(), (first.1+1).try_into().unwrap());
}
/*
 * TWOPARAMETERS returns a tuple of both arguments in a string expression
 * the parentheses need to be trimmed formt he outermost expression but this is done by the grammar. 
 */
fn twoparameters(parameter: &str) -> (String, String)
{
    let first: (u8, u8) =  parentheses(parameter);
    let ending = substring(parameter, (first.1+1).try_into().unwrap(), parameter.len().try_into().unwrap());
    let second: (u8, u8) = parentheses(&ending);
    return (substring(parameter, first.0.try_into().unwrap(), (first.1+1).try_into().unwrap()),
            substring(&ending, second.0.try_into().unwrap(), (second.1+1).try_into().unwrap()));
}

fn solve(equation: &str, independent: char) -> String
{

    let variable = format!("Var (Id \"{}\")", independent );
    
    let part = side(equation, independent);
    if  part != String::from("left") &&  part != String::from("right")
    {
        println!("{}", part);
        return String::from("BAD EQUATION");
    }
    //get first token after equality
    let equivalences = twoparameters(equation);
    // println!("{}", equivalences.0);
    // println!("{}", equivalences.1);

    let mut left = equivalences.0;
    let mut right = equivalences.1;

    left = trimparentheses(&left);
    right = trimparentheses(&right);
    
    //this makes it so that we always solve for the left hand side
    if part == String::from("right")
    {
        let temp = left;
        left = right;
        right = temp;
    }

    let mut location : String = String::from("");
    if part == "left"
    {
        location = twoparameters(equation).0;
    }
    else if part == "right"
    {
        location = twoparameters(equation).1;
    }
    location = trimparentheses(&location);
    //println!("location: {}", location);
    let mut iter = location.split_whitespace();
    let inv = invert(iter.next().unwrap());       

    loop 
    {
        if left == variable
        {
            break;
        }
        // println!("left hand side:  {}", left);
        // println!("right hand side: {}", right);
        let assigned: (String, String) = apply(&right, &left, invert(&args(&left).0), independent);
        left = assigned.0;
        right = assigned.1;

        left = trimparentheses(&left);
        right = trimparentheses(&right);
    }
    let result_equation = format!("Equal ({}) ({})", left, right);
    return result_equation;

    
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
/*
 * APPLY returns a string tuple of both the left 
 * apply_to is the right hand side
 * apply_from is the left hand side
 * the equation is essentially left - inv = right + inv
 * that is the inverse equation is applied to the right 
 * side with the argument NOT including the desired variable (independent)
 */
fn apply(apply_to: &str, apply_from: &str, inverse: &str, independent: char)-> (String, String)
{
    let mut lhs: String = apply_from.to_string();
    let mut rhs: String = apply_to.to_string();

    let expression_number: i8 = find(apply_from, independent);
    if expression_number < 0
    {
        return (String::from("Error: either your variable doesn't exist or this type of equation is not supported"), String::from(""));
    }
    //ok continue
    let argument_count = numberargs(apply_from);
    let mut iter = apply_from.split_whitespace();
    let exp_length = iter.next().unwrap().len();
    
    // base formual
    // lhs = substring(lhs, exp_length, lhs.len().try_into().unwrap());
    // rhs = format!("({} ({}))", inverse, rhs);

    if inverse == "Plus"
    {
        let tuple = twoparameters(&lhs);
        if expression_number == 1
        {
            lhs = tuple.0;
            rhs = format!("Plus ({}) ({})", trimparentheses(&rhs), trimparentheses(&tuple.1));
        }
        else if expression_number == 2
        {
            lhs = format!("Plus ({}) ({})", trimparentheses(&rhs), trimparentheses(&tuple.0));
            rhs = tuple.1;
        }
    }
    if inverse == "Minus"
    {
        let tuple = twoparameters(&lhs);
        if expression_number == 1
        {
            lhs = tuple.0;
            rhs = format!("Minus ({}) ({})", trimparentheses(&rhs), trimparentheses(&tuple.1));
        }
        else if expression_number == 2
        {
            lhs = tuple.1;
            rhs = format!("Minus ({}) ({})", trimparentheses(&rhs), trimparentheses(&tuple.0));
        }
    }
    if inverse == "Times"
    {
        let tuple = twoparameters(&lhs);
        if expression_number == 1
        {
            lhs = tuple.0;
            rhs = format!("Times ({}) ({})", trimparentheses(&rhs), trimparentheses(&tuple.1));
        }
        else if expression_number == 2
        {
            lhs = format!("Times ({}) ({})", trimparentheses(&rhs), trimparentheses(&tuple.1));
            rhs = tuple.0;
        }
    }
    if inverse == "Div"
    {
        let tuple = twoparameters(&lhs);
        if expression_number == 1
        {
            lhs = trimparentheses(&tuple.0);
            rhs = format!("Div ({}) ({})", trimparentheses(&rhs), trimparentheses(&tuple.1));
        }
        else if expression_number == 2
        {
            lhs = trimparentheses(&tuple.1);
            rhs = format!("Div ({}) ({})", trimparentheses(&rhs), trimparentheses(&tuple.0));
        }
    }
    if inverse == "Negate"
    {
        lhs = oneparameter(&lhs);
        rhs = format!("Negate ({})", trimparentheses(&rhs));
    }
    if inverse == "Log" 
    {
        let tuple = twoparameters(&lhs);
        if expression_number == 1 //this means x is the base so  we can use basic exponentiation rules to solve this namely NRoot
        {
            lhs = tuple.0;
            rhs = format!("Sqrt ({}) ({})", trimparentheses(&tuple.1), trimparentheses(&rhs));
        }
        else if expression_number == 2 //this means x is in the power so the right side shoudl eb a logarithm of some base
        {
            lhs = tuple.1;
            rhs = format!("Div (Ln ({})) (Ln ({}))", trimparentheses(&rhs), trimparentheses(&tuple.0));
        }
    }
    if inverse == "Exp10"
    {
        lhs = oneparameter(&lhs);
        rhs = format!("Exp (Int 10) ({})", trimparentheses(&rhs));
    }
    if inverse=="Ln"
    {
        lhs = oneparameter(&lhs);
        rhs = format!("Exp (E) ({})", trimparentheses(&rhs));
    }
    if inverse == "Exp2"
    {
        lhs = oneparameter(&lhs);
        rhs = format!("Exp ({}) (Int 2)", trimparentheses(&rhs));
    }
    if inverse=="ExpN"
    {
        let tuple = twoparameters(&lhs);
        if expression_number == 1
        {
            lhs = format!("Exp ({}) ({})", trimparentheses(&rhs), trimparentheses(&tuple.0));
            rhs = tuple.1;
        }
        else if expression_number == 2
        {
            lhs = tuple.1;
            rhs = format!("Exp ({}) ({})", trimparentheses(&rhs), trimparentheses(&tuple.0));
        }
    }
    if inverse == "Sin"
    {
        lhs = oneparameter(&lhs);
        rhs = format!("Sin ({})", trimparentheses(&rhs));
    }
    if inverse == "ASin"
    {
        lhs = oneparameter(&lhs);
        rhs = format!("ASin ({})", trimparentheses(&rhs));
    }
    if inverse == "Cos"
    {        
        lhs = oneparameter(&lhs);
        rhs = format!("Cos ({})", trimparentheses(&rhs));
    }
    if inverse == "ACos"
    {
        lhs = oneparameter(&lhs);
        rhs = format!("ACos ({})", trimparentheses(&rhs));
    }
    if inverse == "Tan"
    {
        lhs = oneparameter(&lhs);
        rhs = format!("Tan ({})", trimparentheses(&rhs));
    }
    if inverse == "ATan"
    {
        lhs = oneparameter(&lhs);
        rhs = format!("ATan ({})", trimparentheses(&rhs));
    }
    return (trimparentheses(&lhs), trimparentheses(&rhs));
}

/*
 * TRIMPARENTHESES returns the original stirng removing only the first charactetr if it is a opening parentheses and 
 * removing only the final character if it is a closing parentheses. If the first and last characters are neither this 
 * returns the original string. it will also only remove the parenthese if the entire string is surrounded by them
 */
fn trimparentheses(original: &str) -> String
{
    let vec = original.as_bytes(); 
    let open: bool = vec[0] == 40;
    let close: bool = vec[original.len()- 1] == 41;
    let trim: bool = open && close;
    let initial_index: u8 = trim as u8;
    let mut final_index: u8 = original.len().try_into().unwrap();
    final_index -= trim as u8;
    return substring(original, initial_index, final_index);
}