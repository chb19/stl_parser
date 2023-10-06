use std::{io::Write, ops, str::FromStr};

pub type Float = f32;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl ops::Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Mul<Float> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Float) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Div<Float> for Vec3 {
    type Output = Self;
    fn div(self, rhs: Float) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl From<(Float, Float, Float)> for Vec3 {
    fn from(value: (Float, Float, Float)) -> Self {
        Vec3 {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}

impl From<[Float; 3]> for Vec3 {
    fn from(value: [Float; 3]) -> Self {
        Vec3 {
            x: value[0],
            y: value[1],
            z: value[2],
        }
    }
}

impl FromStr for Vec3 {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let by_whitespace = s
            .split_whitespace()
            .map(|x| x.parse::<f32>())
            .collect::<Result<Vec<_>, _>>();

        match by_whitespace {
            Ok(vec) => Vec3::try_from(vec),
            Err(err) => Err(anyhow::anyhow!(
                "Failed to convert string to Vec3!\nErr: {}",
                err
            )),
        }
    }
}

impl TryFrom<Vec<Float>> for Vec3 {
    type Error = anyhow::Error;
    fn try_from(value: Vec<Float>) -> Result<Self, Self::Error> {
        if value.len() != 3 {
            return Err(anyhow::anyhow!(
                "The number of arguments is not equal to 3!"
            ));
        }
        Ok(Vec3 {
            x: value[0],
            y: value[1],
            z: value[2],
        })
    }
}

impl Vec3 {
    pub fn d3(self) -> Float {
        f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }
    pub fn dot(self, rhs: Vec3) -> Float {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    pub fn normalize(self) -> Self {
        let d = self.d3();
        self / d
    }
    // Vec(y, z, x) * Vec(other.z, other.x, other.y) - Vec(z, x, y) * Vec(other.y, other.z, other.x);
    pub fn cross(self, rhs: Vec3) -> Self {
        Vec3::from((self.y, self.z, self.x)) * Vec3::from((rhs.z, rhs.x, rhs.y))
            - Vec3::from((self.z, self.x, self.y)) * Vec3::from((rhs.y, rhs.z, rhs.x))
    }

    pub fn write_le_bytes<W: Write>(&self, writer: &mut W) -> anyhow::Result<()> {
        writer.write_all(&self.x.to_le_bytes())?;
        writer.write_all(&self.y.to_le_bytes())?;
        writer.write_all(&self.z.to_le_bytes())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let v1 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v2 = Vec3 {
            x: 2.0,
            y: 3.0,
            z: 4.0,
        };
        let result = v1 + v2;
        assert_eq!(
            result,
            Vec3 {
                x: 3.0,
                y: 5.0,
                z: 7.0
            }
        );
    }

    #[test]
    fn test_sub() {
        let v1 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v2 = Vec3 {
            x: 2.0,
            y: 3.0,
            z: 4.0,
        };
        let result = v1 - v2;
        assert_eq!(
            result,
            Vec3 {
                x: -1.0,
                y: -1.0,
                z: -1.0
            }
        );
    }

    #[test]
    fn test_mul_scalar() {
        let v = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let scalar = 2.0;
        let result = v * scalar;
        assert_eq!(
            result,
            Vec3 {
                x: 2.0,
                y: 4.0,
                z: 6.0
            }
        );
    }

    #[test]
    fn test_mul_vec3() {
        let v1 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v2 = Vec3 {
            x: 2.0,
            y: 3.0,
            z: 4.0,
        };
        let result = v1 * v2;
        assert_eq!(
            result,
            Vec3 {
                x: 2.0,
                y: 6.0,
                z: 12.0
            }
        );
    }

    #[test]
    fn test_div() {
        let v = Vec3 {
            x: 2.0,
            y: 4.0,
            z: 6.0,
        };
        let scalar = 2.0;
        let result = v / scalar;
        assert_eq!(
            result,
            Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0
            }
        );
    }

    #[test]
    fn test_neg() {
        let v = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let result = -v;
        assert_eq!(
            result,
            Vec3 {
                x: -1.0,
                y: -2.0,
                z: -3.0
            }
        );
    }

    #[test]
    fn test_from_tuple() {
        let tuple = (1.0, 2.0, 3.0);
        let result = Vec3::from(tuple);
        assert_eq!(
            result,
            Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0
            }
        );
    }

    #[test]
    fn test_from_array() {
        let array = [1.0, 2.0, 3.0];
        let result = Vec3::from(array);
        assert_eq!(
            result,
            Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0
            }
        );
    }

    #[test]
    fn test_from_str() {
        let s = "1.0 2.0 3.0";
        let result = Vec3::from_str(s).unwrap();
        assert_eq!(
            result,
            Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0
            }
        );
    }

    #[test]
    fn test_try_from_vec() {
        let vec = vec![1.0, 2.0, 3.0];
        let result = Vec3::try_from(vec);
        assert_eq!(
            result.unwrap(),
            Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0
            }
        );
    }

    #[test]
    fn test_d3() {
        let v = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let result = v.d3();
        assert_eq!(result, f32::sqrt(14.0));
    }

    #[test]
    fn test_dot() {
        let v1 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v2 = Vec3 {
            x: 2.0,
            y: 3.0,
            z: 4.0,
        };
        let result = v1.dot(v2);
        assert_eq!(result, 1.0 * 2.0 + 2.0 * 3.0 + 3.0 * 4.0);
    }

    #[test]
    fn test_normalize() {
        let v = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let length = v.d3();
        let normalized = v.normalize();
        assert_eq!(
            normalized,
            Vec3 {
                x: 1.0 / length,
                y: 2.0 / length,
                z: 3.0 / length
            }
        );
    }

    #[test]
    fn test_cross_simple() {
        let v1 = Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let v2 = Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let _result = v1.cross(v2);
        assert_eq!(
            v1.cross(v2),
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0
            }
        );
        assert_eq!(
            v2.cross(v1),
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0
            }
        );
    }

    #[test]
    fn test_cross() {
        let v1 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v2 = Vec3 {
            x: 2.0,
            y: 3.0,
            z: 4.0,
        };
        let result = v1.cross(v2);
        assert_eq!(
            result,
            Vec3 {
                x: 2.0 - 3.0,
                y: 3.0 * 2.0 - 1.0 * 4.0,
                z: 1.0 * 3.0 - 2.0 * 2.0
            }
        );
    }

    #[test]
    fn test_write_le_bytes() {
        let v = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let mut buffer = Vec::new();
        v.write_le_bytes(&mut buffer).unwrap();
        let expected_bytes: Vec<u8> = vec![
            0x00, 0x00, 0x80, 0x3F, // 1.0 as little-endian f32
            0x00, 0x00, 0x00, 0x40, // 2.0 as little-endian f32
            0x00, 0x00, 0x40, 0x40, // 3.0 as little-endian f32
        ];
        assert_eq!(buffer, expected_bytes);
    }
}
