use std::path::PathBuf;

use clap::Subcommand;

use self::vec3::{Float, Vec3};

pub mod stl_generator;
pub mod stl_parser;
mod vec3;

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum STLFormat {
    Binary,
    ASCII,
}

pub const EPS: f32 = 1e-6;

fn length_is_positive(s: &str) -> Result<f32, String> {
    let len = s.parse::<f32>();
    match len {
        Ok(len) => {
            if len < EPS {
                return Err(format!("Length is: {} < 0:", len));
            }
            Ok(len)
        }
        Err(err) => Err(err.to_string()),
    }
}

#[derive(Debug, Subcommand, Clone)]
pub enum Command {
    Cone {
        #[arg(value_parser = clap::value_parser!(u32).range(3..))]
        n: u32,
        #[arg(value_parser = length_is_positive)]
        r: Float,
        #[arg(value_parser = length_is_positive)]
        h: Float,
        origin: Vec3,
    },
    Cube {
        #[arg(value_parser = length_is_positive)]
        a: Float,
        origin: Vec3,
    },
    Parse {
        input: PathBuf,
    },
}
