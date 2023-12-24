use clap::Parser;

/// Rewrite an MuJoCo MJCF file using the "mjcf" rust library.
///
/// This parses the given MJCF XML file using the "mjcf" rust library,
/// And then serializes it back into an MJCF XML file.
///
/// For more information about the "mjcf" rust library, see:
///     https://crates.io/crates/mjcf
///
/// For more information about the MJCF file format, see:
///     https://mujoco.readthedocs.io/en/stable/XMLreference.html
#[derive(Parser)]
#[command(author, version, about, verbatim_doc_comment)]
struct Args {
    /// MuJoCo "MJCF" XML file
    #[arg()]
    input: String,

    /// If omitted then this will overwrite the input file!
    #[arg()]
    output: Option<String>,
}

fn main() {
    let args = Args::parse();
    let input = std::fs::read_to_string(&args.input).unwrap();
    let value = mjcf::from_str(&input).unwrap();
    let output = mjcf::to_string(&value).unwrap();
    std::fs::write(args.output.unwrap_or(args.input), output).unwrap();
}
