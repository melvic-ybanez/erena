use crate::matrix::Matrix;
use crate::tuples::Tuple;

mod transformations;

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

#[test]
fn test_submatrix() {
    let matrix = Matrix::new33(&[
        1.0, 5.0, 0.0,
        -3.0, 2.0, 7.0,
        0.0, 6.0, -3.0,
    ]);
    assert_eq!(matrix.submatrix(0, 2), Matrix::new22(&[
        -3.0, 2.0,
        0.0, 6.0,
    ]));

    let matrix1 = Matrix::new44(&[
        -6.0, 1.0, 1.0, 6.0,
        -8.0, 5.0, 8.0, 6.0,
        -1.0, 0.0, 8.0, 2.0,
        -7.0, 1.0, -1.0, 1.0,
    ]);
    assert_eq!(matrix1.submatrix(2, 1), Matrix::new33(&[
        -6.0, 1.0, 6.0,
        -8.0, 8.0, 6.0,
        -7.0, -1.0, 1.0,
    ]));
}

#[test]
fn test_minor_for_3x3() {
    let matrix = Matrix::new33(&[
        3.0, 5.0, 0.0,
        2.0, -1.0, -7.0,
        6.0, -1.0, 5.0,
    ]);
    let sub = matrix.submatrix(1, 0);
    assert_eq!(sub.determinant(), 25.0);
    assert_eq!(matrix.minor(1, 0), 25.0);
}

#[test]
fn test_cofactor_for_3x3() {
    let matrix = Matrix::new33(&[
        3.0, 5.0, 0.0,
        2.0, -1.0, -7.0,
        6.0, -1.0, 5.0,
    ]);
    assert_eq!(matrix.minor(0, 0), -12.0);
    assert_eq!(matrix.cofactor(0, 0), -12.0);
    assert_eq!(matrix.minor(1, 0), 25.0);
    assert_eq!(matrix.cofactor(1, 0), -25.0);
}

#[test]
fn test_determinant_for_3x3() {
    let matrix = Matrix::new33(&[
        1.0, 2.0, 6.0,
        -5.0, 8.0, -4.0,
        2.0, 6.0, 4.0,
    ]);
    assert_eq!(matrix.cofactor(0, 0), 56.0);
    assert_eq!(matrix.cofactor(0, 1), 12.0);
    assert_eq!(matrix.cofactor(0, 2), -46.0);
    assert_eq!(matrix.determinant(), -196.0);
}

#[test]
fn test_determinant_for_4x4() {
    let matrix = Matrix::new44(&[
        -2.0, -8.0, 3.0, 5.0,
        -3.0, 1.0, 7.0, 3.0,
        1.0, 2.0, -9.0, 6.0,
        -6.0, 7.0, 7.0, -9.0,
    ]);
    assert_eq!(matrix.cofactor(0, 0), 690.0);
    assert_eq!(matrix.cofactor(0, 1), 447.0);
    assert_eq!(matrix.cofactor(0, 2), 210.0);
    assert_eq!(matrix.cofactor(0, 3), 51.0);
    assert_eq!(matrix.determinant(), -4071.0);
}

#[test]
fn test_invertible() {
    let matrix = Matrix::new44(&[
        6.0, 4.0, 4.0, 4.0,
        5.0, 5.0, 7.0, 6.0,
        4.0, -9.0, 3.0, -7.0,
        9.0, 1.0, 7.0, -6.0,
    ]);
    assert_eq!(matrix.determinant(), -2120.0);
    assert!(matrix.is_invertible());
}

#[test]
fn test_not_invertible() {
    let matrix = Matrix::new44i(&[
        -4, 2, -2, -3,
        9, 6, 2, 6,
        0, -5, 1, -5,
        0, 0, 0, 0,
    ]);
    assert_eq!(matrix.determinant(), 0.0);
    assert!(!matrix.is_invertible());
}

#[test]
fn test_inverse() {
    let m = Matrix::new44i(&[
        -5, 2, 6, -8,
        1, -5, 1, 8,
        7, 7, -6, -7,
        1, -3, 7, 4,
    ]);
    match m.inverse() {
        Some(mi) => {
            assert_eq!(m.determinant(), 532.0);
            assert_eq!(m.cofactor(2, 3), -160.0);
            assert_eq!(mi[(3, 2)], -160.0 / 532.0);
            assert_eq!(m.cofactor(3, 2), 105.0);
            assert_eq!(mi[(2, 3)], 105.0 / 532.0);
            assert_eq!(mi.round_items(5), Matrix::new44(&[
                0.21805, 0.45113, 0.24060, -0.04511,
                -0.80827, -1.45677, -0.44361, 0.52068,
                -0.07895, -0.22368, -0.05263, 0.19737,
                -0.52256, -0.81391, -0.30075, 0.30639
            ]))
        }
        None => assert!(false)
    }

    let m = Matrix::new44i(&[
        8, -5, 9, 2,
        7, 5, 6, 1,
        -6, 0, 9, 6,
        -3, 0, -9, -4,
    ]);
    match m.inverse() {
        Some(mi) => assert_eq!(mi.round_items(5), Matrix::new44(&[
            -0.15385, -0.15385, -0.28205, -0.53846,
            -0.07692, 0.12308, 0.02564, 0.03077,
            0.35897, 0.35897, 0.43590, 0.92308,
            -0.69231, -0.69231, -0.76923, -1.92308
        ])),
        None => assert!(false)
    }

    let m = Matrix::new44i(&[
        9, 3, 0, 9,
        -5, -2, -6, -3,
        -4, 9, 6, 4,
        -7, 6, 6, 2,
    ]);
    match m.inverse() {
        Some(mi) => assert_eq!(mi.round_items(5), Matrix::new44(&[
            -0.04074, -0.07778, 0.14444, -0.22222,
            -0.07778, 0.03333, 0.36667, -0.33333,
            -0.02901, -0.14630, -0.10926, 0.12963,
            0.17778, 0.06667, -0.26667, 0.33333,
        ])),
        None => assert!(false)
    }
}

#[test]
fn test_multiply_product_by_inverse() {
    let m = Matrix::new44i(&[
        3, -9, 7, 3,
        3, -8, 2, -9,
        -4, 4, 4, 1,
        -6, 5, -1, 1,
    ]);
    let m1 = Matrix::new44i(&[
        8, 2, 2, 2,
        3, -1, 7, 0,
        7, 0, 5, 4,
        6, -2, 0, 5,
    ]);
    let product = &m * &m1;
    match m1.inverse() {
        Some(inverse) => assert_eq!(&product * &inverse, m),
        None => assert!(false)
    }
}
