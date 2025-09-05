pub mod bounds;
pub mod vector2;

trait VectorOps {
    type Output;

    fn len(&self) -> Self::Output;

    fn normalize(&self) -> Self;
}
