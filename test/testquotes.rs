fn main()
{
    println!(remove_quotes("\"R\"=\frac{\"V\"}{\"I\"}"));
}

fn remove_quotes(original: &str) -> String
{
    String::from(riginal.replace("\"", ""));
}