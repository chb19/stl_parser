use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, Read, Write},
    path::PathBuf,
};

use super::vec3::Vec3;

use anyhow::{anyhow, Context};

use super::vec3::Float;

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub normal: Vec3,
    pub vertices: [Vec3; 3],
}

pub type Triangles = Vec<Triangle>;

pub struct StlParser;

impl StlParser {
    pub fn read_stl(path: PathBuf) -> anyhow::Result<Triangles> {
        match StlParser::read_binary_stl(path.clone()) {
            Ok(triangles) => Ok(triangles),
            Err(err_bin) => match StlParser::read_ascii_stl(path) {
                Ok(triangles) => Ok(triangles),
                Err(err) => {
                    Err(anyhow::anyhow!(
                    "Failed to read STL file in either binary or ascii format!\nErrors:\n{}\n{}",
                    err_bin,
                    err
                ))
                }
            },
        }
    }

    fn read_binary_stl(path: PathBuf) -> anyhow::Result<Triangles> {
        let mut file = File::open(path.clone())
            .with_context(|| format!("Failed to open file \'{}\'!", path.display()))?;
        let mut triangles = Triangles::new();
        let mut header = [0u8; 80];
        file.read_exact(&mut header)
            .with_context(|| format!("Failed to read header from binary STL!"))?;

        let mut num_triangles_bytes = [0u8; 4];
        file.read_exact(&mut num_triangles_bytes)
            .with_context(|| format!("Failed to read number of triangles from binary STL!"))?;

        let num_triangles = u32::from_le_bytes(num_triangles_bytes);

        for _i in 0..num_triangles {
            let mut buffer = [0u8; 50];
            file.read_exact(&mut buffer)
                .with_context(|| format!("Failed to read triangle from binary STL!"))?;

            let normal: Vec3 = (
                f32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]),
                f32::from_le_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]),
                f32::from_le_bytes([buffer[8], buffer[9], buffer[10], buffer[11]]),
            )
                .into();

            let mut vertices = [[0.0; 3]; 3];
            for i in 0..3 {
                let offset = 12 + i * 12;
                vertices[i][0] = f32::from_le_bytes([
                    buffer[offset],
                    buffer[offset + 1],
                    buffer[offset + 2],
                    buffer[offset + 3],
                ]);
                vertices[i][1] = f32::from_le_bytes([
                    buffer[offset + 4],
                    buffer[offset + 5],
                    buffer[offset + 6],
                    buffer[offset + 7],
                ]);
                vertices[i][2] = f32::from_le_bytes([
                    buffer[offset + 8],
                    buffer[offset + 9],
                    buffer[offset + 10],
                    buffer[offset + 11],
                ]);
            }

            triangles.push(Triangle {
                normal,
                vertices: [vertices[0].into(), vertices[1].into(), vertices[2].into()],
            })
        }

        Ok(triangles)
    }

    pub fn write_to_binary_file(path: PathBuf, triangles: Triangles) -> anyhow::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path.clone())
            .with_context(|| {
                format!(
                    "Failed to create a file for writing!\nPath: \'{}\'",
                    path.display()
                )
            })?;

        let header = [0u8; 80];
        file.write_all(&header)
            .with_context(|| format!("Failed to write header into binary STL!"))?;

        let num_triangles = triangles.len() as u32;
        file.write_all(&num_triangles.to_le_bytes())
            .with_context(|| format!("Failed to write number of triangles into binary STL!"))?;

        for triangle in triangles {
            triangle
                .normal
                .write_le_bytes(&mut file)
                .with_context(|| format!("Failed to write normal of triangle into binary STL!"))?;

            for vertex in triangle.vertices {
                vertex.write_le_bytes(&mut file).with_context(|| {
                    format!("Failed to write vertex of triangle into binary STL!")
                })?;
            }
            // Write the attribute byte count (set to zero)
            let attribute_byte_count: [u8; 2] = [0, 0];
            file.write_all(&attribute_byte_count).with_context(|| {
                format!("Failed to write attribute byte count into binary STL!")
            })?;
        }

        Ok(())
    }

    fn read_ascii_stl(path: PathBuf) -> anyhow::Result<Triangles> {
        let file = File::open(path.clone())
            .with_context(|| format!("Failed to open file \'{}\'!", path.display()))?;

        let mut lines = io::BufReader::new(file).lines();

        // the first line starts with solid <name>
        match lines.next() {
            Some(Ok(line)) => {
                if !line.starts_with("solid ") {
                    return Err(anyhow!("STL file does not start with \'solid\'!"));
                }
            }
            Some(Err(err)) => {
                return Err(anyhow!("Failed to read the first line!\nError: {}", err));
            }
            None => return Err(anyhow!("Failed to read from the file!")),
        }

        let mut lines = lines
            .filter_map(|line| line.ok())
            .filter(|line| !line.trim().is_empty());

        // facet normal -0.01905 -0.770147 0.637582
        //     outer loop
        //     vertex 3.29426 -0.2921 0.067036
        //     vertex 3.35026 -0.216682 0.159808
        //     vertex 3.11981 -0.142593 0.242416
        //     endloop
        // endfacet
        let mut triangles = Triangles::new();
        while let Some(line) = lines.next() {
            let line = line.trim();
            if line.starts_with("facet normal ") {
                let pts = line
                    .split_whitespace()
                    .skip(2)
                    .map(|num| num.parse())
                    .collect::<Result<Vec<Float>, _>>();

                let pts = match pts {
                    Ok(pts) => pts,
                    Err(err) => {
                        return Err(anyhow!(
                            "Failed to parse normal coordinates! Err: {:?}",
                            err
                        ))
                    }
                };

                // outer loop
                lines.next();
                let mut vertices = Vec::new();

                for _ in 0..3 {
                    if let Some(line) = lines.next() {
                        let pts = line
                            .split_whitespace()
                            .skip(1)
                            .map(|num| num.parse())
                            .collect::<Result<Vec<Float>, _>>();

                        let pts = match pts {
                            Ok(pts) => pts,
                            Err(err) => {
                                return Err(anyhow!(
                                    "Failed to parse normal coordinates! Err: {:?}",
                                    err
                                ))
                            }
                        };

                        let vertex: Vec3 = pts.try_into()?;
                        vertices.push(vertex);
                    }
                }
                triangles.push(Triangle {
                    normal: pts.try_into()?,
                    vertices: vertices.try_into().unwrap(),
                });

                // endloop
                lines.next();
                // endfacet
                lines.next();
            }
        }

        Ok(triangles)
    }

    pub fn write_to_ascii_file(path: PathBuf, triangles: Triangles) -> anyhow::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path.clone())
            .with_context(|| {
                format!(
                    "Failed to create a file for writing!\nPath: \'{}\'",
                    path.display()
                )
            })?;

        writeln!(file, "solid RustStlParser")?;
        for triangle in triangles {
            let normal = triangle.normal;
            writeln!(
                file,
                " facet normal {:.} {:.} {:.}",
                normal.x, normal.y, normal.z
            )?;
            writeln!(file, "  outer loop")?;
            for vertex in triangle.vertices {
                writeln!(
                    file,
                    "  vertex {:.} {:.} {:.}",
                    vertex.x, vertex.y, vertex.z
                )?;
            }
            writeln!(file, "  endloop")?;
            writeln!(file, " endfacet")?;
            writeln!(file)?;
        }
        writeln!(file, "endsolid RustStlParser")?;
        Ok(())
    }
}
