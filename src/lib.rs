mod atosl;

mod demangle;

use atosl::GroupAddress;
use neon::prelude::*;

fn parse_address_string(address: &str) -> Result<u64, anyhow::Error> {
    if address.starts_with("0x") {
        let value = address.trim_start_matches("0x");
        let value = u64::from_str_radix(value, 16)?;
        Ok(value)
    } else {
        let value = address.parse::<u64>()?;
        Ok(value)
    }
}

fn parse(mut cx: FunctionContext) -> JsResult<JsObject> {
    let params = cx.argument::<JsObject>(0)?;
    let params_file: Handle<JsString> = params.get(&mut cx, "file")?;
    let params_load_address: Handle<JsString> = params.get(&mut cx, "load_address")?;
    let params_addresses: Handle<JsArray> = params.get(&mut cx, "addresses")?;
    let arg_offset_text_segment = cx.argument_opt(1);
    let mut params_offset_text_segment = false;
    if let Some(arg_offset_text_segment) = arg_offset_text_segment {
        params_offset_text_segment = arg_offset_text_segment.downcast_or_throw::<JsBoolean, FunctionContext>(&mut cx).map(|op| op.value(&mut cx))?;
    }
    let file:String = params_file.value(&mut cx);
    let load_address:u64 = parse_address_string(&params_load_address.value(&mut cx)).unwrap();
    let addresses = params_addresses
        .to_vec(&mut cx).unwrap()
        .into_iter()
        .map(|v| v.downcast_or_throw::<JsString, FunctionContext>(&mut cx).map(|c|parse_address_string(&c.value(&mut cx)).unwrap()))
        .collect::<Result<Vec<_>, _>>()?;
    let result = atosl::print_addresses(
        &file,
        load_address,
        addresses,
        params_offset_text_segment
    );
    return match result {
        Ok(parse_result) => {
            let response_array = JsArray::new(&mut cx, parse_result.len() as u32);
            let result_obj = cx.empty_object();
            for (i, result_instance) in parse_result.iter().enumerate() {
                let obj = cx.empty_object();
                let address_number = cx.number(result_instance.address as f64);
                let result_string = cx.string(&result_instance.result);
                obj.set(&mut cx, "address", address_number).unwrap();
                obj.set(&mut cx, "result", result_string).unwrap();
                response_array.set(&mut cx, i as u32, obj).unwrap();
            }
            let success = cx.boolean(true);
            let data = response_array;
            let message = cx.null();
            result_obj.set(&mut cx, "success", success).unwrap();
            result_obj.set(&mut cx, "data", data).unwrap();
            result_obj.set(&mut cx, "message", message).unwrap();
            return Ok(result_obj);
        },
        Err(err) => {
            let result_obj = cx.empty_object();
            let success = cx.boolean(false);
            let data = cx.empty_array();
            let message = cx.string(err.to_string());
            result_obj.set(&mut cx, "success", success).unwrap();
            result_obj.set(&mut cx, "data", data).unwrap();
            result_obj.set(&mut cx, "message", message).unwrap();
            Ok(result_obj)
        },
    }
    
}

fn transform_group_address(obj: &Handle<JsObject>, cx: &mut FunctionContext) -> GroupAddress {
    let load_address: Handle<JsString> = obj.get(cx, "load_address").unwrap();
    let params_addresses: Handle<JsArray> = obj.get(cx, "addresses").unwrap();
    let addresses = params_addresses
        .to_vec(cx).unwrap()
        .into_iter()
        .map(|v| v.downcast_or_throw::<JsString, FunctionContext>(cx).map(|c|parse_address_string(&c.value(cx)).unwrap()))
        .collect::<Result<Vec<_>, _>>().unwrap();
    let load_address_u64:u64 = parse_address_string(&load_address.value(cx)).unwrap();
    return GroupAddress {
        load_address: load_address_u64,
        addresses: addresses
    }
}

fn group_parse(mut cx: FunctionContext) -> JsResult<JsObject> {
    let params = cx.argument::<JsObject>(0)?;
    let params_file: Handle<JsString> = params.get(&mut cx, "file")?;
    // let params_load_address: Handle<JsString> = params.get(&mut cx, "load_address")?;
    let params_addresses: Handle<JsArray> = params.get(&mut cx, "addresses")?;
    let arg_offset_text_segment = cx.argument_opt(1);
    let mut params_offset_text_segment = false;
    if let Some(arg_offset_text_segment) = arg_offset_text_segment {
        params_offset_text_segment = arg_offset_text_segment.downcast_or_throw::<JsBoolean, FunctionContext>(&mut cx).map(|op| op.value(&mut cx))?;
    }
    let file:String = params_file.value(&mut cx);
    // let load_address:u64 = parse_address_string(&params_load_address.value(&mut cx)).unwrap();
    let addresses = params_addresses
        .to_vec(&mut cx).unwrap()
        .into_iter()
        .map(|v| v.downcast_or_throw::<JsObject, FunctionContext>(&mut cx).map(|c|transform_group_address(&c, &mut cx)))
        .collect::<Result<Vec<_>, _>>()?;
    let result = atosl::parse_file_addresses(
        &file,
        addresses,
        params_offset_text_segment
    );
    return match result {
        Ok(parse_result) => {
            let response_array = JsArray::new(&mut cx, parse_result.len() as u32);
            let result_obj = cx.empty_object();
            for (i, result_instance) in parse_result.iter().enumerate() {
                let obj = cx.empty_object();
                let address_number = cx.number(result_instance.address as f64);
                let result_string = cx.string(&result_instance.result);
                obj.set(&mut cx, "address", address_number).unwrap();
                obj.set(&mut cx, "result", result_string).unwrap();
                response_array.set(&mut cx, i as u32, obj).unwrap();
            }
            let success = cx.boolean(true);
            let data = response_array;
            let message = cx.null();
            result_obj.set(&mut cx, "success", success).unwrap();
            result_obj.set(&mut cx, "data", data).unwrap();
            result_obj.set(&mut cx, "message", message).unwrap();
            return Ok(result_obj);
        },
        Err(err) => {
            let result_obj = cx.empty_object();
            let success = cx.boolean(false);
            let data = cx.empty_array();
            let message = cx.string(err.to_string());
            result_obj.set(&mut cx, "success", success).unwrap();
            result_obj.set(&mut cx, "data", data).unwrap();
            result_obj.set(&mut cx, "message", message).unwrap();
            Ok(result_obj)
        },
    }
    
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("parse", parse)?;
    cx.export_function("groupParse", group_parse)?;
    Ok(())
}
