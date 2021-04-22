#[macro_use]
extern crate rutie;

mod parser;

use rutie::{Class, Object, AnyObject, RString, VM, Integer, Array};
use parser::{parse, BenCodeExpr};

class!(RutieExample);

methods!(
    RutieExample,
    _rtself,

    fn pub_parse(input: RString) -> AnyObject {
        let ruby_string = input.
        map_err(|e| VM::raise_ex(e)).
        unwrap();

        let result = parse(ruby_string.to_str());
        match result {
            Ok(r) => convert_to_ruby(r),
            Err(e) => RString::new_utf8(&e).into()
        }
    }
);

fn convert_to_ruby(bencode: BenCodeExpr) -> AnyObject {
    match bencode {
        BenCodeExpr::Int(e) => Integer::new(e).into(),
        BenCodeExpr::List(values) => {
            let mut result = Array::with_capacity(values.len());
            for value in values {
                result.push(convert_to_ruby(value));
            }
            result.into()
        }
    }
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_rutie_ruby_example() {
    Class::new("BenCode", None).define(|klass| {
        klass.def_self("parse", pub_parse);
    });
}
