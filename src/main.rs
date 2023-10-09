use std::path::PathBuf;

use clap::Parser;

use stl_parser::utils::{stl_generator::StlGenerator, stl_parser::StlParser, Command, StlFormat};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// output path
    #[arg(long, short = 'o', value_hint = clap::ValueHint::FilePath)]
    output: Option<PathBuf>,

    /// format of the output file
    #[arg(long)]
    #[clap(value_enum)]
    output_format: StlFormat,

    /// supported commands
    #[command(subcommand)]
    command: Command,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let output = if let Some(output) = args.output {
        output
    } else {
        match args.command.clone() {
            Command::Cone { .. } => PathBuf::from("Cone.stl"),
            Command::Cube { .. } => PathBuf::from("Cube.stl"),
            Command::Parse { input } => {
                let mut input = input
                    .to_string_lossy()
                    .split_once('.')
                    .unwrap()
                    .0
                    .to_string();
                input.push_str("_out.stl");
                PathBuf::from(input)
            }
        }
    };

    match args.command {
        Command::Cone { n, r, h, origin } => {
            StlGenerator::generate_cone_stl(n, r, h, origin, output, args.output_format)?
        }
        Command::Cube { a, origin } => {
            StlGenerator::generate_cube_stl(a, origin, output, args.output_format)?
        }
        Command::Parse { input } => {
            let triangles = StlParser::read_stl(input)?;

            match args.output_format {
                StlFormat::ASCII => StlParser::write_to_ascii_file(output, triangles)?,
                StlFormat::Binary => StlParser::write_to_binary_file(output, triangles)?,
            }
        }
    }

    Ok(())
}
