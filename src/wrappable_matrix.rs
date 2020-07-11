use ndarray::Array2;

#[derive(Debug, PartialEq)]
pub struct WrappableMatrix {
    matrix: Array2<f64>,
}

impl WrappableMatrix {
    fn new(matrix: Array2<f64>) -> Self {
        Self { matrix }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr2;

    mod helpers {
        use super::*;

        pub fn array2x3_with_zeros() -> Array2<f64> {
            let ary: Array2<f64> = Array2::zeros((2, 3));
            assert_eq!(ary, arr2(&[[0., 0., 0.], [0., 0., 0.]]));

            ary
        }
    }

    #[test]
    fn wrappable_matrix_new_can_construct_from_ndarray_Array2_f64() {
        let ary1 = helpers::array2x3_with_zeros();
        let ary2 = helpers::array2x3_with_zeros();

        let wrappable_matrix_new = WrappableMatrix::new(ary1);
        let wrappable_matrix_struct = WrappableMatrix { matrix: ary2 };

        assert_eq!(wrappable_matrix_new, wrappable_matrix_struct);
    }
}
