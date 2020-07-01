#[macro_use]
extern crate rutie;

use rutie::{Class, Object, RString, VM};

class!(MatrixRs);

methods!(
    MatrixRs,
    _itself,

    fn pub_self_brackets(input: RString) -> RString {
        let ruby_string = input.
          map_err(|e| VM::raise_ex(e) ).
          unwrap();

        RString::new_utf8(
          &ruby_string.
          to_string().
          chars().
          rev().
          collect::<String>()
        )
    }
);

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_matrix_rs() {
    Class::new("MatrixRs", None).define(|itself| {
        itself.def_self("[]", pub_self_brackets);
    });
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
