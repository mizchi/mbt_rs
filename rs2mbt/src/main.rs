use std::io::Read;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = if args.len() > 1 {
        std::fs::read_to_string(&args[1]).expect("Failed to read input file")
    } else {
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf).expect("Failed to read stdin");
        buf
    };
    print!("{}", rs2mbt::to_moonbit(&input));
}
