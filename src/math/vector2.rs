use crate::math::VectorOps;
use std::ops::{Add, Div, Mul, Sub};

#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct Vector2<T> {
    x: T,
    y: T,
}

impl Vector2<f32> {
    pub fn unit() -> Self {
        Self { x: 1.0, y: 1.0 }
    }
}

impl Vector2<f64> {
    pub fn unit() -> Self {
        Self { x: 1.0, y: 1.0 }
    }
}

impl Vector2<i8> {
    pub fn unit() -> Self {
        Self { x: 1, y: 1 }
    }
}

impl Vector2<i16> {
    pub fn unit() -> Self {
        Self { x: 1, y: 1 }
    }
}

impl Vector2<i32> {
    pub fn unit() -> Self {
        Self { x: 1, y: 1 }
    }
}

impl Vector2<i64> {
    pub fn unit() -> Self {
        Self { x: 1, y: 1 }
    }
}

impl VectorOps for Vector2<f32> {
    type Output = f32;

    fn len(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn normalize(&self) -> Vector2<f32> {
        *self / self.len()
    }
}

impl VectorOps for Vector2<f64> {
    type Output = f64;

    fn len(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn normalize(&self) -> Vector2<f64> {
        *self / self.len()
    }
}

impl<T> Add for Vector2<T>
where
    T: Add<T, Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Sub for Vector2<T>
where
    T: Sub<T, Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Mul<T> for Vector2<T>
where
    T: Mul<T, Output = T>,
    T: Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> Div<T> for Vector2<T>
where
    T: Div<T, Output = T>,
    T: Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Vector2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::math::vector2::Vector2;

    #[test]
    fn test_add() {
        let v1 = Vector2 { x: 3.0, y: 4.0 };
        let v2 = Vector2 { x: 7.0, y: 6.0 };

        let res = v1 + v2;

        assert_eq!(res, Vector2 { x: 10., y: 10. });
    }
}
