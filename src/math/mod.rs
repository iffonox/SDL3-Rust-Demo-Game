pub mod vector2;
pub mod bounds;

trait VectorOps {
    type Output;

    fn len(&self) -> Self::Output;

    fn normalize(&self) -> Self;
}
