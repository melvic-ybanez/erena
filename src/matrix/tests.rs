use crate::tuples::Tuple;
use crate::matrix::Matrix;

#[test]
fn test_4x4_matrix_creation() {
    let matrix = Matrix::new44(&[
        1.0, 2.0, 3.0, 4.0,
        5.5, 6.5, 7.5, 8.5,
        9.0, 10.0, 11.0, 12.0,
        13.5, 14.5, 15.5, 16.5,
    ]);
    assert_eq!(matrix[(0, 0)], 1.0);
    assert_eq!(matrix[(0, 3)], 4.0);
    assert_eq!(matrix[(1, 0)], 5.5);
    assert_eq!(matrix[(1, 2)], 7.5);
    assert_eq!(matrix[(2, 2)], 11.0);
    assert_eq!(matrix[(3, 0)], 13.5);
    assert_eq!(matrix[(3, 2)], 15.5);
}

#[test]
fn test_2x2_matrix() {
    let m = Matrix::new22(&[
        -3.0, 5.0,
        1.0, -2.0
    ]);
    assert_eq!(m[(0, 0)], -3.0);
    assert_eq!(m[(0, 1)], 5.0);
    assert_eq!(m[(1, 0)], 1.0);
    assert_eq!(m[(1, 1)], -2.0);
}

#[test]
fn test_3x3_matrix() {
    let m = Matrix::new33(&[
        -3.0, 5.0, 0.0,
        1.0, -2.0, -7.0,
        0.0, 1.0, 1.0
    ]);
    assert_eq!(m[(0, 0)], -3.0);
    assert_eq!(m[(1, 1)], -2.0);
    assert_eq!(m[(2, 2)], 1.0);
}

#[test]
fn test_matrix_equality() {
    let elems = &[
        1.0, 2.0, 3.0, 4.0,
        5.0, 6.0, 7.0, 8.0,
        9.0, 8.0, 7.0, 6.0,
        5.0, 4.0, 3.0, 2.0,
    ];
    let m = Matrix::new44(elems);
    let m1 = Matrix::new44(elems);
    assert_eq!(m, m1);
}

#[test]
fn test_matrix_inequality() {
    let m = Matrix::new44(&[
        1.0, 2.0, 3.0, 4.0,
        5.0, 6.0, 7.0, 8.0,
        9.0, 8.0, 7.0, 6.0,
        5.0, 4.0, 3.0, 2.0,
    ]);
    let m1 = Matrix::new44(&[
        2.0, 3.0, 4.0, 5.0,
        6.0, 7.0, 8.0, 9.0,
        8.0, 7.0, 6.0, 5.0,
        4.0, 3.0, 2.0, 1.0
    ]);
    assert_ne!(m, m1);
}

#[test]
fn test_matrix_multiplication() {
    let m1 = Matrix::new44(&[
        1.0, 2.0, 3.0, 4.0,
        5.0, 6.0, 7.0, 8.0,
        9.0, 8.0, 7.0, 6.0,
        5.0, 4.0, 3.0, 2.0,
    ]);
    let m2 = Matrix::new44(&[
        -2.0, 1.0, 2.0, 3.0,
        3.0, 2.0, 1.0, -1.0,
        4.0, 3.0, 6.0, 5.0,
        1.0, 2.0, 7.0, 8.0,
    ]);
    assert_eq!(m1 * m2, Matrix::new44(&[
        20.0, 22.0, 50.0, 48.0,
        44.0, 54.0, 114.0, 108.0,
        40.0, 58.0, 110.0, 102.0,
        16.0, 26.0, 46.0, 42.0,
    ]));
}

#[test]
fn test_matrix_tuple_multiplication() {
    let matrix = Matrix::new44(&[
        1.0, 2.0, 3.0, 4.0,
        2.0, 4.0, 4.0, 2.0,
        8.0, 6.0, 4.0, 1.0,
        0.0, 0.0, 0.0, 1.0,
    ]);
    let tuple = Tuple::new(1.0, 2.0, 3.0, 1.0);
    assert_eq!(matrix * tuple, Tuple::new(18.0, 24.0, 33.0, 1.0));
}

#[test]
fn test_identity_matrix() {
    fn matrix() -> Matrix {
        Matrix::new44(&[
            0.0, 1.0, 2.0, 4.0,
            1.0, 2.0, 4.0, 8.0,
            2.0, 4.0, 8.0, 16.0,
            4.0, 8.0, 16.0, 32.0,
        ])
    }
    assert_eq!(matrix() * Matrix::id44(), matrix());
}

#[test]
fn test_transposition() {
    let matrix = Matrix::new44(&[
        0.0, 9.0, 3.0, 0.0,
        9.0, 8.0, 0.0, 8.0,
        1.0, 8.0, 5.0, 3.0,
        0.0, 0.0, 5.0, 8.0,
    ]);
    assert_eq!(matrix.transpose(), Matrix::new44(&[
        0.0, 9.0, 1.0, 0.0,
        9.0, 8.0, 8.0, 0.0,
        3.0, 0.0, 5.0, 5.0,
        0.0, 8.0, 3.0, 8.0,
    ]));
}

#[test]
fn test_2x2_determinant() {
    let matrix = Matrix::new_nxn(2, &[
        1.0, 5.0,
        -3.0, 2.0,
    ]);
    assert_eq!(matrix.determinant(), 17.0);
}