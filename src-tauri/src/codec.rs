#[allow(unused)]
#[cfg(not(test))]
mod dataset;
#[allow(unused)]
#[cfg(not(test))]
mod script;

#[cfg(not(test))]
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 3 {
        eprintln!("Usage: lmocodec.exe [decode|encode] [input file] [output file]",);
        std::process::exit(1);
    }
    let mode = &args[1];
    if mode != "decode" && mode != "encode" {
        eprintln!("Invalid mode: {}", mode);
        std::process::exit(1);
    }
    let input_file_path = &args[2];
    let output_file_path = &args[3];
    if mode == "decode" {
        let input_file = std::fs::read(input_file_path).unwrap();
        let output = script::file::dat::cipher_to_text(&input_file);
        std::fs::write(output_file_path, output).unwrap();
    } else {
        let input_file = std::fs::read_to_string(input_file_path).unwrap();
        let output = script::file::dat::text_to_cipher(&input_file);
        std::fs::write(output_file_path, output).unwrap();
    }
}
