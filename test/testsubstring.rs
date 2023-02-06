fn main()
{                           //0123456789
    println!("{}", substring("0000111000", 4, 7));
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