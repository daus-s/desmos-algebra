fn main() 
{
    println!("{}", hello_string("xyz"));
    println!("{}", conditional_strings("string"));
    println!("{}", conditional_strings("char vector"));

}



fn hello_string(_x: &str) -> &str {
    return "howdy";
}

fn conditional_strings(x: &str) -> bool
{
    x == "string"
}