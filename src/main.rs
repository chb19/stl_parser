use std::path::PathBuf;

use clap::Parser;

use stl_parser::utils::{stl_generator::STLGenerator, stl_parser::STLParser, Command, STLFormat};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// output path
    #[arg(long, short = 'o', value_hint = clap::ValueHint::FilePath)]
    output: Option<PathBuf>,

    /// format of the output file
    #[arg(long)]
    #[clap(value_enum)]
    output_format: STLFormat,

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
            STLGenerator::generate_cone_stl(n, r, h, origin, output, args.output_format)?
        }
        Command::Cube { a, origin } => {
            STLGenerator::generate_cube_stl(a, origin, output, args.output_format)?
        }
        Command::Parse { input } => {
            let triangles = STLParser::read_stl(input)?;

            match args.output_format {
                STLFormat::ASCII => STLParser::write_ascii_stl(output, triangles)?,
                STLFormat::Binary => STLParser::write_binary_stl(output, triangles)?,
            }
        }
    }

    Ok(())
}
