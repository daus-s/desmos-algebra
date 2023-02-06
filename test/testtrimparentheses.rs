use std::convert::TryInto;

fn main()
{
    assert_eq!("NaN", trimparentheses("(NaN)"));
    assert_eq!("xxxxx", trimparentheses("xxxxx"));
    assert_eq!("()", trimparentheses("(())"));
}


fn trimparentheses(original: &str) -> String
{
    let vec = original.as_bytes(); 
    let open: bool = vec[0] == 40;
    let close: bool = vec[original.len()- 1] == 41;
    let initial_index: u8 = open as u8;
    let mut final_index: u8 = original.len().try_into().unwrap();
    final_index -= close as u8;
    return substring(original, initial_index, final_index);
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