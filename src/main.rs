use clap::Parser;

mod crc32;
mod file;

#[derive(Parser)]
#[command()]
struct Args {
    /// Input file path
    name: String,

    /// Hashing algorithm
    algo: String,

    /// Hash (hex)
    hash: String,
}

fn main() {
    let args = Args::parse();

    match args.algo.as_str() {
        "crc32" => {
            crc32::find(&args.name, &args.hash);
        }
        _ => {
            println!(
                "Hashing algorithm {} is currently not supported.",
                args.algo
            );
        }
    }
}
