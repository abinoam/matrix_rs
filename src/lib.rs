#[macro_use]
extern crate rutie;

#[macro_use]
extern crate lazy_static;
extern crate ndarray;

mod args_treating;
mod wrappable_matrix;

use rutie::rubysys::class;
use rutie::types::{Argc, Value};
use rutie::util::str_to_cstring;
use rutie::{AnyObject, Array, Integer};
use rutie::{Class, Object, RString, VerifiedObject};
use std::mem;
use wrappable_matrix::WrappableMatrix;

wrappable_struct!(WrappableMatrix, MatrixWrapper, MATRIX_WRAPPER_INSTANCE);

class!(MatrixRs);

impl VerifiedObject for MatrixRs {
    fn is_correct_type<T: Object>(object: &T) -> bool {
        Class::from_existing("MatrixRs").case_equals(object)
    }

    fn error_message() -> &'static str {
        "Error converting to MatrixRs"
    }
}

pub extern "C" fn pub_self_brackets(argc: Argc, argv: *const AnyObject, _: AnyObject) -> AnyObject {
    let args = Value::from(0);

    unsafe {
        let p_argv: *const Value = mem::transmute(argv);

        class::rb_scan_args(argc, p_argv, str_to_cstring("*").as_ptr(), &args)
    };

    let matrix = WrappableMatrix::from(Array::from(args));
    Class::from_existing("MatrixRs").wrap_data(matrix, &*MATRIX_WRAPPER_INSTANCE)
}

methods!(
    MatrixRs, // Rutie struct (class)
    rtself,   // Rutie self (instance) - methods will receive it as arg
    fn pub_self_empty(row_count: Integer, col_count: Integer) -> Array {
        // build a fake empty Array for testing

        let row_count = match row_count {
            Ok(row_count) => row_count.to_i64() as usize,
            Err(_e) => return Array::new(),
        };

        let col_count = match col_count {
            Ok(col_count) => col_count.to_i64() as usize,
            Err(_e) => return Array::new(),
        };

        let mut array = Array::with_capacity(row_count);

        for _row in 0..row_count {
            let mut current_row = Array::with_capacity(col_count);

            for _col in 0..col_count {
                current_row.push(Integer::new(0));
            }

            array.push(current_row);
        }

        array
    }

    fn pub_to_s() -> RString {
        let matrix_str = rtself.get_data(&*MATRIX_WRAPPER_INSTANCE).to_s();

        RString::from(matrix_str)
    }
);

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_matrix_rs() {
    Class::new("MatrixRs", None).define(|klass| {
        klass.def_self("[]", pub_self_brackets);
        klass.def_self("empty", pub_self_empty);

        klass.def("to_s", pub_to_s);
    });
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
