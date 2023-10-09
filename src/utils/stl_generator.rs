use std::{f32::consts::PI, path::PathBuf};

use super::{
    stl_parser::{StlParser, Triangle, Triangles},
    vec3::{Float, Vec3},
    StlFormat,
};

pub struct StlGenerator;

impl StlGenerator {
    fn generate_cube(a: Float, origin: Vec3) -> Triangles {
        let mut v: Vec<Vec3> = Vec::new();

        for dx in [-1.0, 1.0] {
            for dy in [-1.0, 1.0] {
                for dz in [-1.0, 1.0] {
                    v.push(
                        (
                            origin.x + dx * a / 2.0,
                            origin.y + dy * a / 2.0,
                            origin.z + dz * a / 2.0,
                        )
                            .into(),
                    );
                }
            }
        }

        
        Triangles::from([
            Triangle {
                vertices: [v[4], v[0], v[6]],
                normal: (v[0] - v[4]).cross(v[6] - v[4]).normalize(),
            },
            Triangle {
                vertices: [v[2], v[6], v[0]],
                normal: (v[6] - v[2]).cross(v[0] - v[2]).normalize(),
            },
            Triangle {
                vertices: [v[5], v[1], v[7]],
                normal: (v[1] - v[5]).cross(v[7] - v[5]).normalize(),
            },
            Triangle {
                vertices: [v[3], v[7], v[1]],
                normal: (v[1] - v[3]).cross(v[7] - v[3]).normalize(),
            },
            Triangle {
                vertices: [v[7], v[3], v[6]],
                normal: (v[3] - v[7]).cross(v[6] - v[7]).normalize(),
            },
            Triangle {
                vertices: [v[2], v[6], v[3]],
                normal: (v[6] - v[2]).cross(v[3] - v[2]).normalize(),
            },
            Triangle {
                vertices: [v[4], v[5], v[6]],
                normal: (v[5] - v[4]).cross(v[6] - v[4]).normalize(),
            },
            Triangle {
                vertices: [v[7], v[6], v[5]],
                normal: (v[5] - v[7]).cross(v[6] - v[7]).normalize(),
            },
            Triangle {
                vertices: [v[0], v[1], v[2]],
                normal: (v[1] - v[0]).cross(v[2] - v[0]).normalize(),
            },
            Triangle {
                vertices: [v[3], v[2], v[1]],
                normal: (v[2] - v[3]).cross(v[1] - v[3]).normalize(),
            },
            Triangle {
                vertices: [v[4], v[5], v[0]],
                normal: (v[5] - v[4]).cross(v[0] - v[4]).normalize(),
            },
            Triangle {
                vertices: [v[1], v[0], v[5]],
                normal: (v[5] - v[1]).cross(v[0] - v[1]).normalize(),
            },
        ])
    }


    pub fn generate_cube_stl(
        a: Float,
        origin: Vec3,
        output: PathBuf,
        format: StlFormat,
    ) -> anyhow::Result<()> {
        let triangles = StlGenerator::generate_cube(a, origin);
        match format {
            StlFormat::ASCII => StlParser::write_to_ascii_file(output, triangles)?,
            StlFormat::Binary => StlParser::write_to_binary_file(output, triangles)?,
        }
        Ok(())
    }

    fn generate_cone(n: usize, r: Float, h: Float, origin: Vec3) -> Triangles {
        let mut v: Vec<Vec3> = Vec::new();

        let top = origin + Vec3::from((0.0f32, 0.0f32, h));

        for i in 0..=n {
            v.push(Vec3::from((
                (1.0 * (i as f32) / (n as f32) * 2.0f32 * PI).cos() * r,
                (1.0 * (i as f32) / (n as f32) * 2.0f32 * PI).sin() * r,
                0.0f32,
            )));
        }

        let mut triangles = Triangles::new();
        for i in 0..n {
            triangles.push(Triangle {
                normal: (v[(i + 1) % n] - v[i]).cross(v[i] - top).normalize(),
                vertices: [v[i], v[(i + 1) % n], top],
            })
        }

        for i in 0..n - 2 {
            triangles.push(Triangle {
                normal: (v[i + 2] - v[0]).cross(v[i + 1] - v[0]).normalize(),
                vertices: [v[0], v[i + 1], v[i + 2]],
            });
        }
        triangles
    }

    pub fn generate_cone_stl(
        n: u32,
        r: Float,
        h: Float,
        origin: Vec3,
        output: PathBuf,
        format: StlFormat,
    ) -> anyhow::Result<()> {
        let triangles = StlGenerator::generate_cone(n as usize, r, h, origin);

        match format {
            StlFormat::ASCII => StlParser::write_to_ascii_file(output, triangles)?,
            StlFormat::Binary => StlParser::write_to_binary_file(output, triangles)?,
        }
        Ok(())
    }
}
