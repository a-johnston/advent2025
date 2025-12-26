extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn setup_problems(max: TokenStream) -> TokenStream {
    let max_day: usize = format!("{max}").parse().unwrap();
    let mods: String = (1..=max_day).map(|i| format!("mod day{i};\n")).collect();
    let parts: String = (1..=max_day)
        .map(|i| format!("&day{i}::PARTS,\n"))
        .collect();
    format!("{mods}\nstatic PROBLEMS: &'static [&'static [Part<'static>]] = &[\n{parts}];")
        .parse()
        .unwrap()
}
