use std::collections::HashMap;

use napi::{CallContext, JsBoolean, JsObject, JsString, Result};
use svg_simple_parser::Element;

// #[inline(always)]
// fn  deserialize_attributes(ctx:&CallContext,attributes_hash:&JsObject)->Result<JsObject>{
//   let mut ele = ctx.env.create_array()?;
//   for (key,&value) in attributes_hash.iter() {
//     ele.set_named_property(key, ctx.env.create_string(value)?)?;
//   }
//   Ok(ele)
// }

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

#[inline(always)]
fn deserialize_element<'a>(ctx: &CallContext, root: JsObject) -> Result<Element<'a>> {
    let ele_type: &'static str = string_to_static_str(
        root.get_named_property::<JsString>("type")?
            .into_utf8()?
            .into_owned()?,
    );

    let attributes: JsObject = root
        .get_named_property("attributes")
        .unwrap_or(ctx.env.create_object()?);

    let mut attributes_hash: HashMap<String, &str> = HashMap::new();
    let keys = attributes.get_property_names()?;
    for i in 0..keys.get_array_length()? {
        let key = keys.get_element::<JsString>(i)?.into_utf8()?.into_owned()?;
        let value = string_to_static_str(
            attributes
                .get_named_property::<JsString>(key.as_str())?
                .into_utf8()?
                .into_owned()?,
        );
        attributes_hash.insert(key, value);
    }

    let children: JsObject = root
        .get_named_property("children")
        .unwrap_or(ctx.env.create_array()?);

    let len = children.get_array_length()?;
    let mut child_array = vec![];
    for i in 0..len {
        let a: JsObject = children.get_element(i)?;
        child_array.push(deserialize_element(&ctx, a)?);
    }

    let ele = Element::new_width_children((ele_type, attributes_hash, child_array));
    Ok(ele)
}

#[js_function(2)]
pub fn stringify_fn(ctx: CallContext) -> Result<JsString> {
    let argument = ctx.get::<JsObject>(0)?;
    let is_pretty = ctx.get::<JsBoolean>(1)?.get_value().unwrap_or(false);
    let root: Element = deserialize_element(&ctx, argument)?;
    let svg = if is_pretty {
        svg_simple_parser::stringify_pretty(&root)
    } else {
        svg_simple_parser::stringify(&root)
    };

    ctx.env.create_string(&svg)
}
