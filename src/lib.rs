#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

mod parse;
mod stringify;

use napi::{JsObject, Result};


#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {

    exports.create_named_method("parse", parse::parse_fn)?;

    exports.create_named_method("stringify", stringify::stringify_fn)?;
    Ok(())
}


