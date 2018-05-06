#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

struct Matrix {
    values: Vec<f64>,
    rows: i32,
    cols: i32
}

impl Matrix {
    fn new(rows: i32, cols: i32, values: Vec<f64>) -> Matrix {
        if values.len() != rows * cols {
            panic!("Number of Values in Matrix didn't match it's rows * cols");
        }

        Matrix {
            values: values,
            rows: rows,
            cols: cols
        }
    }
}
