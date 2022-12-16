extern crate core;

use once_cell::sync::Lazy;
use ext_php_rs::args::Arg;
use ext_php_rs::builders::FunctionBuilder;
use ext_php_rs::convert::IntoZval;
use ext_php_rs::flags::DataType;
use ext_php_rs::prelude::*;
use ext_php_rs::types::{Zval};
use ext_php_rs::zend::ExecuteData;


static INSTANCE: Lazy<reqwest::blocking::Client> = Lazy::new(|| {
    reqwest::blocking::Client::builder()
        .http2_prior_knowledge()
        .build().unwrap()
});

// TODO  pub extern "C" fn multi_reqwest(ex: &mut ExecuteData, retval: &mut Zval){
//
// }

pub extern "C" fn reqwest(ex: &mut ExecuteData, retval: &mut Zval)
{
    let mut request_arg = Arg::new("request", DataType::String);
    if ex
        .parser()
        .arg(&mut request_arg)
        .parse()
        .is_err()
    {
        return;
    }

    let mut request: String = match request_arg.val() {
        Some(val) => val,
        None => {
            println!("None failed:");
            return;
        }
    };

    // todo prevent create new every one. use shared one for support multiplexing between php requests
    // https://github.com/seanmonstar/reqwest/discussions/1470

    let response = match INSTANCE.get(request).send() { // TODO pass whole request
        Ok(r) => r,
        Err(err) => {
            println!("Request failed: {}", err.to_string());
            return;
        },
    };

    // let body = response.text()?;
    let result = match response.text() {
        Ok(r) => r,
        Err(err) => {
            println!("Request failed: {}", err.to_string());
            return;
        },
    };
    // let body = NAME;

    result.set_zval(retval, false);
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    // TODO module.startup_function()
    module.function(
        FunctionBuilder::new("reqwest", reqwest)
            .arg(Arg::new("request", DataType::String)) // pass raw http request. todo psr7 request object?!
            .returns(DataType::String, false, false) // return raw http response. todo psr7 response?!
            // TODO pass some client configuration https://docs.rs/reqwest/latest/reqwest/blocking/struct.ClientBuilder.html#method.http2_prior_knowledge
            .build()
            .unwrap()
    )
}
