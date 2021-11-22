use std::{collections::HashMap, convert::TryInto};

use napi::{CallContext, JsObject, JsString, Result};
use svg_simple_parser::Element;

#[inline(always)]
fn serialize_attributes(
    ctx: &CallContext,
    attributes_hash: &HashMap<String, &str>,
) -> Result<JsObject> {
    let mut ele = ctx.env.create_object()?;
    for (key, &value) in attributes_hash.iter() {
        ele.set_named_property(key, ctx.env.create_string(value)?)?;
    }
    Ok(ele)
}

#[inline(always)]
fn serialize_element(ctx: &CallContext, root: &Element) -> Result<JsObject> {
    let Element {
        ele_type,
        attributes,
        children,
    } = root;
    let mut ele = ctx.env.create_object()?;
    ele.set_named_property("type", ctx.env.create_string(ele_type)?)?;
    ele.set_named_property("attributes", serialize_attributes(&ctx, attributes)?)?;
    let mut eles = ctx.env.create_array_with_length(children.len())?;
    for (i, ele) in children.iter().enumerate() {
        eles.set_element(i.try_into().unwrap(), serialize_element(&ctx, ele)?)?;
    }
    ele.set_named_property("children", eles)?;
    Ok(ele)
}

#[js_function(1)]
pub fn parse_fn(ctx: CallContext) -> Result<JsObject> {
    let argument = ctx.get::<JsString>(0)?.into_utf8()?;
    // let arr:&str = argument.as_str();
    let (_, root) = svg_simple_parser::parse(argument.as_str()?).unwrap();

    serialize_element(&ctx, &root)
}
