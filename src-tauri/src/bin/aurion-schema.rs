fn main() -> Result<(), String> {
    let arguments: Vec<String> = std::env::args().skip(1).collect();

    let check = match arguments.as_slice() {
        [] => false,
        [argument] if argument == "--check" => true,
        _ => return Err("Usage: aurion-schema [--check]".into()),
    };

    aurion_lib::export_schema(check)
}
