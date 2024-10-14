// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a


fn trim_me(input: &str) -> String {
    input.trim().to_string()
}

fn compose_me(input: &str) -> String {
    let mut result = String::from(input);
    result.push_str(" world!");
    result
}

fn replace_me(input: &str) -> String {
    input.replace("cars", "balloons").to_string()
}
