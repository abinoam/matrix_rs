use crate::args_treating::ArgsTreating;
use ndarray::Array2;
use rutie::{AnyException, Array, Exception, Float, Object, VM};

#[derive(Debug, PartialEq)]
pub struct WrappableMatrix {
    matrix: Array2<f64>,
}

impl WrappableMatrix {
    fn new(matrix: Array2<f64>) -> Self {
        Self { matrix }
    }

    pub fn to_s(&self) -> String {
        self.matrix.to_string()
    }

    pub fn dot(&self, other: &Self) -> Self {
        let matrix = self.matrix.dot(&other.matrix);
        Self { matrix }
    }
}

impl From<rutie::Array> for WrappableMatrix {
    fn from(ary: rutie::Array) -> Self {
        let mut vec: Vec<f64> = Vec::new();
        let rows = ary.length();

        ary.into_iter()
            .map(|row_any_obj| row_any_obj.try_convert_to::<Array>())
            .map(|result| result.unwrap_or_rb_raise())
            .for_each(|row_ary| {
                row_ary
                    .into_iter()
                    .map(|col_any_obj| col_any_obj.try_convert_to::<Float>())
                    .map(|result| result.unwrap_or_rb_raise())
                    .for_each(|col_float| vec.push(col_float.to_f64()))
            });

        let cols = vec.len() / rows;

        match Array2::from_shape_vec((rows, cols), vec) {
            Ok(matrix) => Self { matrix },
            Err(err) => {
                VM::raise_ex(AnyException::new(
                    "StandardError",
                    Some(&format!("ndarray error: {:?}", err)),
                ));
                unreachable!()
            }
        }
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
