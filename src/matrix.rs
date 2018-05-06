pub struct Matrix {
    values: Vec<f64>,
    rows: i32,
    cols: i32
}

impl Matrix {
    pub fn new(rows: i32, cols: i32, values: Vec<f64>) -> Matrix {
        if values.len() != (rows * cols) as usize {
            panic!("Number of Values in Matrix didn't match it's rows * cols");
        }

        Matrix {
            values: values,
            rows: rows,
            cols: cols
        }
    }

    pub fn get_rows(&self) -> i32 { self.rows }

    pub fn get_cols(&self) -> i32 { self.cols }
    
    pub fn get_value(&self, row: i32, col: i32) -> f64 {
        if ! self.coordinate_in_matrix(row, col) {
            panic!("Coordinate not in Matrix");
        }

        self.values[(row * col) as usize]
    }

    pub fn eliminate(&self, row: i32, col: i32) -> Matrix {
        if ! self.coordinate_in_matrix(row, col) {
            panic!("Coordinate not in Matrix");
        }
        
        let mut values: Vec<f64> = Vec::with_capacity(((self.rows-1) * (self.cols-1)) as usize);

        for i in 0..self.rows {
            if i == row {
                continue;
            }
            for j in 0..self.cols {
                if j == col {
                    continue
                }
                values.push(self.get_value(i, j));
            }
        }

        Matrix::new(self.rows-1, self.cols-1, values)
    }

    fn coordinate_in_matrix(&self, row: i32, col: i32) -> bool {
        row >= 0 && row < self.rows && col >= 0 && col < self.cols
    }
}


fn determinante_laplace(m: &Matrix) -> f64 {
    if m.get_cols() == 2 && m.get_rows() == 2 {
       return m.get_value(0, 0) * m.get_value(1, 1) - m.get_value(0,1) * m.get_value(1, 0);
    }
    let mut det = 0_f64;
    for i in 0..m.cols {
        if i != 0 {
            det += i as f64 * determinante_laplace(m) * if i | 1 == 1 { -1.0 } else { 1.0 };
        }
    }
    det
}
