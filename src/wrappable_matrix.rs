use rutie::Integer;
use crate::args_treating::ArgsTreating;
use ndarray::Array2;
use rutie::{AnyException, Array, Exception, Float, Object, VM};

#[derive(Debug, PartialEq)]
pub enum WrappableMatrix {
    MFloat(Array2<f64>),
    MInt(Array2<i64>),
}

pub enum IntFloat {
    Int(i64),
    Float(f64),
}

impl WrappableMatrix {
    pub fn to_s(&self) -> String {
        match self {
            Self::MFloat(ary_f64) => ary_f64.to_string(),
            Self::MInt(ary_i64) => ary_i64.to_string(),
        }
    }

    pub fn dot(&self, other: &Self) -> Self {
        match self {
            Self::MFloat(self_f64) => match other {
                Self::MFloat(other_f64) => Self::MFloat(self_f64.dot(other_f64)),
                _ => {
                    VM::raise_ex(AnyException::new(
                        "StandardError",
                        Some("Different element types RMatrix (not float)"),
                    ));
                    unreachable!()
                }
            },
            Self::MInt(self_i64) => match other {
                Self::MInt(other_i64) => Self::MInt(self_i64.dot(other_i64)),
                _ => {
                    VM::raise_ex(AnyException::new(
                        "StandardError",
                        Some("Different element types RMatrix (not integer)"),
                    ));
                    unreachable!()
                }
            },
        }
    }

    pub fn fetch(&self, row: usize, col: usize) -> IntFloat {
        match self {
            Self::MFloat(self_f64) => { IntFloat::Float(self_f64[[row, col]]) },
            Self::MInt(self_i64) => { IntFloat::Int(self_i64[[row, col]]) },

        }
    }
}

impl From<rutie::Array> for WrappableMatrix {
    fn from(ary: rutie::Array) -> Self {
        let mut vec_f: Vec<f64> = Vec::new();
        let mut vec_i: Vec<i64> = Vec::new();
        let rows = ary.length();

        ary.into_iter()
            .map(|row_any_obj| row_any_obj.try_convert_to::<Array>())
            .map(|result| result.unwrap_or_rb_raise())
            .for_each(|row_ary| {
                row_ary
                    .into_iter()
                    .for_each(|col_any_obj| {
                        if let Ok(rb_float) = col_any_obj.try_convert_to::<Float>() {
                            vec_f.push(rb_float.to_f64())
                        } else if let Ok(rb_int) = col_any_obj.try_convert_to::<Integer>() {
                            vec_i.push(rb_int.to_i64())
                        // Repeat try_convert_to to catch Exception
                        } else if let Err(exception) = col_any_obj.try_convert_to::<Integer>() {
                            VM::raise_ex(exception);
                            unreachable!();
                        }
                    })
            });

        if vec_f.len() > 0 && vec_i.len() > 0 {
            VM::raise_ex(AnyException::new(
                "StandardError",
                Some("Not all elements are the same type"),
            ));
            unreachable!()
        }

        if vec_f.len() > vec_i.len() {
            let cols = vec_f.len() / rows;

            match Array2::from_shape_vec((rows, cols), vec_f) {
                Ok(matrix) => Self::MFloat(matrix),
                Err(err) => {
                    VM::raise_ex(AnyException::new(
                        "StandardError",
                        Some(&format!("ndarray error: {:?}", err)),
                    ));
                    unreachable!()
                }
            }
        } else {
            let cols = vec_i.len() / rows;

            match Array2::from_shape_vec((rows, cols), vec_i) {
                Ok(matrix) => Self::MInt(matrix),
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr2;

    #[test]
    fn mfloat_wrappable_matrix_may_multiply() {
        let ary1 = arr2(&[[1.1, 2.2, 3.3], [4.4, 5.5, 6.6]]);
        let ary2 = arr2(&[[1.1, 2.2], [3.3, 4.4], [5.5, 6.6]]);
        let result = arr2(&[[26.62, 33.88], [59.29, 77.44]]);

        let mfloat1 = WrappableMatrix::MFloat(ary1);
        let mfloat2 = WrappableMatrix::MFloat(ary2);
        let expected_result = WrappableMatrix::MFloat(result);

        assert_eq!(expected_result, mfloat1.dot(&mfloat2));
    }

    #[test]
    fn mint_wrappable_matrix_may_multiply() {
        let ary1 = arr2(&[[1, 2, 3], [4, 5, 6]]);
        let ary2 = arr2(&[[1, 2], [3, 4], [5, 6]]);
        let result = arr2(&[[22, 28], [49, 64]]);

        let mint1 = WrappableMatrix::MInt(ary1);
        let mint2 = WrappableMatrix::MInt(ary2);
        let expected_result = WrappableMatrix::MInt(result);

        assert_eq!(expected_result, mint1.dot(&mint2));
    }
}
