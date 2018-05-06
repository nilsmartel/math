pub mod matrix;


#[cfg(test)]
mod tests {
    use matrix::Matrix;
    use matrix::determinante_laplace;

    #[test]
    fn determinant_test() {
        let m = Matrix::new(3, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);

        assert_eq!(determinante_laplace(&m), 0.0);
    }
}
