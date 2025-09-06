pub mod bounds;
pub mod vector2;

pub trait VectorOps {
    type Output;

    fn len(&self) -> Self::Output;

    fn normal(&self) -> Self;
}
