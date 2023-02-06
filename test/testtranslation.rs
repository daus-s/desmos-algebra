use std::collections::HashMap;

fn translation_table() -> HashMap<&'static str, &'static str>
{
    HashMap::from([
    ("Equal", "ARGUMENT1=ARGUMENT2"),
    ("Minus","ARGUMENT1-ARGUMENT2"),
    ("Plus","ARGUMENT1+ARGUMENT2"),
    ("Times","ARGUMENT1\\cdotARGUMENT2"),
    ("Div","\\frac{ARGUMENT1}{ARGUMENT2}"),
    ("Negate","-ARGUMENT"),
    ("Exp","^ARGUMENT2"),
    ("Fact","ARGUMENT!"),
    ("Sqrt","\\sqrt{ARGUMENT}"),
    ("NRt","\\sqrt[ARGUMENT1]{ARGUMENT2}"),
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

fn main()
{
    let table = translation_table();
    println!("{}", (dbg!(table.get("Minus")).unwrap().replace("ARGUMENT1", "a").replace("ARGUMENT2", "b")));
}