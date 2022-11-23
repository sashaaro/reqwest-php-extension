#![macro_use]

#[macro_export]
macro_rules! parse_argument {
    ($arg:expr, $name:literal) => {
        match $arg.val() {
            Some(val) => val,
            None => {
                throw_default!(format!("Invalid value given for argument `{}`.", $name));
            }
        }
    };
    ($arg:expr, $name:literal, $default:expr) => {
        match $arg.val() {
            Some(val) => val,
            None => $default,
        }
    };
}

#[macro_export]
macro_rules! set_return {
    ($result:expr, $retval:expr) => {
        if let Err(e) = $result.set_zval($retval, false) {
            let e: ::ext_php_rs::exception::PhpException = e.into();
            e.throw().expect("Failed to throw exception");
        }

        return;
    };
}
