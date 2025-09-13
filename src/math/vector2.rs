use crate::math::VectorOps;
use sdl3::render::FPoint;
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(PartialEq, Debug, Default, Clone, Copy, Deserialize, Serialize)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
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

    fn normal(&self) -> Vector2<f32> {
        *self / self.len()
    }
}

impl VectorOps for Vector2<f64> {
    type Output = f64;

    fn len(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn normal(&self) -> Vector2<f64> {
        *self / self.len()
    }
}

impl<T> Neg for Vector2<T>
where
	T: Neg<Output = T>
{
	type Output = Self;

	fn neg(self) -> Self::Output {
		Self {
			x: -self.x,
			y: -self.y,
		}
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

impl<T> AddAssign for Vector2<T>
where
	T: AddAssign<T>,
{
	fn add_assign(&mut self, rhs: Self) {
		self.x += rhs.x;
		self.y += rhs.y;
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

impl<T> SubAssign for Vector2<T>
where
	T: SubAssign<T>,
{
	fn sub_assign(&mut self, rhs: Self) {
		self.x -= rhs.x;
		self.y -= rhs.y;
	}
}

impl<T> Mul for Vector2<T>
where
	T: Mul<T, Output = T>,
	T: Add<T, Output = T>,
	T: Copy,
{
	type Output = T;

	fn mul(self, rhs: Self) -> Self::Output {
		self.x * self.y + rhs.x * rhs.y
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

impl<T> MulAssign<T> for Vector2<T>
where
	T: MulAssign<T>,
	T: Copy,
{
	fn mul_assign(&mut self, rhs: T) {
		self.x *= rhs;
		self.y *= rhs;
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

impl<T> DivAssign<T> for Vector2<T>
where
	T: DivAssign<T>,
	T: Copy,
{
	fn div_assign(&mut self, rhs: T) {
		self.x /= rhs;
		self.y /= rhs;
	}
}

impl<T> From<FPoint> for Vector2<T>
where
    T: From<f32>,
{
    fn from(value: FPoint) -> Self {
        Self {
            x: T::from(value.x),
            y: T::from(value.y),
        }
    }
}

impl<T> From<Vector2<T>> for FPoint
where
    T: Into<f32>,
{
    fn from(value: Vector2<T>) -> Self {
        Self {
            x: T::into(value.x),
            y: T::into(value.y),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::math::vector2::Vector2;
	use crate::math::VectorOps;

	#[test]
    fn test_add() {
        let v1 = Vector2 { x: 3.0, y: 4.0 };
        let v2 = Vector2 { x: 7.0, y: 6.0 };

        let res = v1 + v2;

        assert_eq!(res, Vector2 { x: 10., y: 10. });
    }

	#[test]
	fn test_normal() {
		let v1 = Vector2 { x: 3.0, y: 4.0 };
		let v2 = Vector2 { x: -3.0, y: -4.0 };

		let res1 = v1.normal();
		let res2 = v2.normal();

		assert_eq!(1.0, res1.len());
		assert_eq!(1.0, res2.len());
	}
}
