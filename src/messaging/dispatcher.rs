use core::num;
use std::path::Path;
use std::thread;
use std::time::Duration;
use chrono::format;
use log::info;
use oci_spec::runtime::Spec;
use uuid::Uuid;
use walkdir::WalkDir;
use wasmedge_sdk::{Caller, WasmValue, host_function, Engine,Executor};
use wasmedge_sdk::error::HostFuncError;
use crate::messaging::message::Message;
use crate::messaging::{redis_utils, shim_listener,remote_messenger};
use crate::messaging::remote_messenger::listener;
use crate::utils::{oci_utils, snapshot_utils};
use rayon::prelude::*;
extern crate libc;

pub static mut OCI_SPEC:Option<Spec> = None;
pub static mut BUNDLE_PATH:Option<String> = None;

#[host_function]
pub fn func_connect(caller: Caller, input: Vec<WasmValue>) -> Result<Vec<WasmValue>, HostFuncError> {

    let mut mem = caller.memory(0).unwrap();
    let arg1_ptr = input[0].to_i32() as u32;
    let arg1_len = input[1].to_i32() as u32;
    // let num = input[2].to_i32() as u32;
    //println!("External function input length {}",arg1_len);
    let payload = mem.read(arg1_ptr, arg1_len).expect("fail to get string");
    // println!("payload: {:#?}", payload);

    // let mut message_obj: Message = serde_json::from_str(&external_function_type).unwrap();
    let mut external_function_type = "func_b.wasm";
    // let tmp_str = format!("fb{}.wasm", num);
    // if num == 1 {
    //     external_function_type = "fb1.wasm";
    // } else if num == 2 {
    //     external_function_type = "fb2.wasm";
    // } else if num == 3 {
    //     external_function_type = "fb3.wasm";
    // } else {
    //     external_function_type = &tmp_str
    // }
    // println!("num: {}", num);
    // println!("Function target {}",external_function_type);

    let socket_path: String;

    // listener(payload).expect("TODO: panic message");

    let mut ext_func_result:String;
    unsafe {
        //let bundle_path = BUNDLE_PATH.clone().unwrap().rsplitn(3, '/').nth(2).unwrap().to_string()+"/";

        // socket_path= find_container_path(bundle_path.clone(), external_function_type);
        let io_start = chrono::offset::Utc::now();
        socket_path = snapshot_utils::find_container_path_parallel(BUNDLE_PATH.as_deref().unwrap_or(""), external_function_type);
        let io_end = chrono::offset::Utc::now();
        println!("io duration: {}", io_end - io_start);
    }
    println!("socket path {}",socket_path);
    if socket_path.is_empty() {
        // ext_func_result = connect_to_queue(external_function_type.replace(".wasm",""), message_obj.payload);
        // ext_func_result= unsafe {shim_listener::send_shared_memory(payload, external_function_type).unwrap()};

    } else {
        unsafe {
            // ext_func_result= unsafe {shim_listener::send_shared_memory(payload).unwrap()};
            ext_func_result = shim_listener::connect_unix_socket(payload, socket_path).unwrap();
        }
        //UNTIL HERE
    }



    ext_func_result = "100".to_string();

   /* //let payload = mem.read(arg1_ptr, arg1_len).unwrap();
    //listener("test.wasm".to_string(), payload).expect("TODO: panic message");
    println!("printing final result {}",ext_func_result);
    let bytes = ext_func_result.as_bytes();
    let len = bytes.len();
    mem.write(bytes, arg1_ptr).unwrap();
    */
    
    
    Ok(vec![WasmValue::from_i32(100 as i32)])

}




fn connect_to_queue(channel :String, fn_target_input:String) -> String{

    let fn_source_id = Uuid::new_v4().simple().to_string();
    let fn_source_id_copy = fn_source_id.clone();
    let _ = redis_utils::publish_message(Message::new(fn_source_id,
                                                      channel, fn_target_input,chrono::offset::Utc::now().to_string())).unwrap();
    let result = redis_utils::_subscribe(fn_source_id_copy.as_str());
    return result.payload;
}


fn find_container_path(path:String, function_name:String) -> String {
    for file in WalkDir::new(path.clone()).into_iter().filter_map(|file| file.ok()) {
        let file_name = file.file_name().to_str().unwrap();
        if file.metadata().unwrap().is_file() && file_name=="config.json" {
            info!("oci config spec found: {}", file.path().display());
            let c_path = file.path().display().to_string().replace("/config.json","");
            let spec = oci_utils::load_spec(c_path.clone()).unwrap();
            let args = oci_utils::arg_to_wasi(&spec);
            let c_path_formatted=args.first().unwrap().to_string().replace("/","");
            if c_path_formatted==function_name && Path::new(&(c_path.clone()+".sock")).exists(){
                return c_path;
            }
        }
    }
    return String::new();
}



