#[cfg(test)]
mod tests {
    use crate::shapes::Shape;
    use crate::matrix::Matrix;

    #[test]
    fn test_create_group() {
        let group = Shape::empty_group();
        assert_eq!(group.transformation, Matrix::id44());
        assert!(group.geometry.is_empty());
    }

    #[test]
    fn test_shape_parent() {
        let shape = Shape::test();
        assert!(shape.parent.is_none());
    }
}