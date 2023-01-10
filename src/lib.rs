extern crate core;

use std::collections::HashMap;
use once_cell::sync::Lazy;
use ext_php_rs::args::Arg;
use ext_php_rs::builders::FunctionBuilder;
use ext_php_rs::convert::IntoZval;
use ext_php_rs::flags::DataType;
use ext_php_rs::{info_table_end, info_table_row, info_table_start};
use ext_php_rs::prelude::*;
use ext_php_rs::types::{Zval};
use ext_php_rs::zend::{ExecuteData, ModuleEntry};
use reqwest::header::HeaderMap;
use reqwest::Method;


static INSTANCE: Lazy<reqwest::blocking::Client> = Lazy::new(|| {
    println!("Init reqwest instance!");
    reqwest::blocking::Client::builder()
        .http2_prior_knowledge()
        .build().unwrap()
});

pub extern "C" fn reqwest(ex: &mut ExecuteData, retval: &mut Zval)
{
    let mut methodArg = Arg::new("method", DataType::String);
    let mut urlArg = Arg::new("url", DataType::String);
    let mut headersArg = Arg::new("headers", DataType::Array);

    if ex
        .parser()
        .arg(&mut methodArg)
        .arg(&mut urlArg)
        .arg(&mut headersArg)
        .parse()
        .is_err()
    {
        return;
    }

    let mut methodStr: String = match methodArg.val() {
        Some(val) => val,
        None => {
            println!("Invalid method");
            return;
        }
    };
    let mut url: String = match urlArg.val() {
        Some(val) => val,
        None => {
            println!("Invalid url");
            return;
        }
    };

    let mut headers: HashMap<String, String> = match headersArg.val() {
        Some(val) => val,
        None => {
            println!("Invalid headers");
            return;
        }
    };


    // todo prevent create new every one. use shared one for support multiplexing between php requests
    // https://github.com/seanmonstar/reqwest/discussions/1470

    let mut method: Method = Method::from_bytes(methodStr.as_bytes()).unwrap();

    let mut reqHeaders = HeaderMap::try_from(&headers).unwrap();
    let request_builder = INSTANCE.request(method, url).headers(reqHeaders);

    let response = match request_builder.send() { // TODO pass whole request
        Ok(r) => r,
        Err(err) => {
            println!("Request failed: {}", err.to_string());
            return;
        },
    };

    let mut res = HashMap::new();

    res.insert("url",response.url().to_string());
    //res["headers"] = response.headers();
    res.insert("status", response.status().to_string());

    let body = match response.text() {
        Ok(r) => r,
        Err(err) => {
            println!("Request failed: {}", err.to_string());
            return;
        },
    };
    res.insert("body", body);


    res.set_zval(retval, false).expect("TODO: panic message");
}

pub extern "C" fn php_module_info(_module: *mut ModuleEntry) {
    info_table_start!();
    info_table_row!("Reqwest", "enabled");
    info_table_end!();
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    // TODO module.startup_function()
    module
        .info_function(php_module_info)
        .function(
        FunctionBuilder::new("reqwest", reqwest)
            .arg(Arg::new("method", DataType::String)) // pass raw http request. todo psr7 request object?!
            .arg(Arg::new("url", DataType::String)) // pass raw http request. todo psr7 request object?!
            .arg(Arg::new("headers", DataType::Array)) // pass raw http request. todo psr7 request object?!
            .returns(DataType::Array, false, false) // return raw http response. todo psr7 response?!
                // TODO pass some client configuration https://docs.rs/reqwest/latest/reqwest/blocking/struct.ClientBuilder.html#method.http2_prior_knowledge
            .build()
            .unwrap()
        )
}
