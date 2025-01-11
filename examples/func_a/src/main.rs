use serde_json::value::Index;
use tokio::join;
use wasmedge_http_req::request;
use chrono::{Duration};
use core::{f32, time};
use std::{future::Future, string, sync::{Arc, Mutex}};
/*
sudo ctr -n k8s.io run --rm --runtime=io.containerd.cwasi.v1 \
--annotation cwasi.secondary.function=false --net-host=true docker.io/library/funca:latest \
funca /func_a.wasm 11 11 11 11

export HTTP_PROXY=socks5://127.0.0.1:8888
export HTTPS_PROXY=socks5://127.0.0.1:8888

docker build -t funca .
docker save -o funca.tar funca:latest
sudo ctr -n k8s.io images rm docker.io/library/funca:latest
sudo ctr -n k8s.io images import funca.tar

docker build -t funca . && docker save -o funca.tar funca:latest && sudo ctr -n mysp images rm docker.io/library/funca:latest && \
    sudo ctr -n mysp images import funca.tar



    docker build -t fa .
    docker save -o fa.tar docker.io/library/fa:latest
    sudo ctr -n mysp images import fa.tar
*/

fn main(){
    println!("Greetings from func_a {}",chrono::offset::Utc::now());
    cwasi_function();
}


#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn cwasi_function() -> i32 {
    unsafe {
        let args: Vec<String> = std::env::args().collect();
        println!("args: {:?} read at {}", args,chrono::offset::Utc::now());
        let storage_ip = std::env::var("STORAGE_IP").expect("Error: STORAGE_URL not found");
        println!("Value of STORAGE_IP: {}", storage_ip);

        println!("Downloading file started at {}",chrono::offset::Utc::now());
        let file:String = args[2].parse().unwrap();
        let mut writer = Vec::new(); //container for body of a response
        let res = request::get("http://".to_owned()+&storage_ip+ &"/files/".to_owned()+&file, &mut writer).unwrap();
        println!("Downloading finished at {}",chrono::offset::Utc::now());
        let response_string = &std::str::from_utf8_unchecked(&writer);
        // println!("response string: {:#?}", response_string);
        println!("Data copied to string at {}",chrono::offset::Utc::now());
        //println!("GET");
        println!("Response Status: {} {}", res.status_code(), res.reason());
        //println!("Headers {}", res.headers());

        // process_response(response_string);
        let index: i32 = std::env::var("FUNCTIONS_NUM").expect("Error: FUNCTIONS_NUM not found").parse().unwrap();
        println!("Value of FUNCTIONS_NUM: {}", index);
        // let mut duration:Duration = Duration::seconds(0);
        let start = chrono::offset::Utc::now();
        // send_request(response_string.clone());
        
        // let rt = Runtime::new().unwrap();
        // let _guard = rt.enter();
        // for i in 0..index {
        //     let start = chrono::offset::Utc::now();
        //     let duration_func_microsec =send_request(response_string.clone()).replace("Received from client at : ", "").replace("\n", "");
        //     let duration_b=chrono::Duration::microseconds(duration_func_microsec.parse::<i64>().unwrap());

        //     duration = duration+duration_b;
        //     let seconds = duration.num_microseconds().unwrap() as f64/1000000 as f64;
        //     let throughput = (i+1) as f64/ seconds as f64;
        //     println!("throughput: {} index {}", throughput,i);
        // }
        // println!("Result  {} sent, Duration {} ms", index,duration.to_owned().num_milliseconds());
        // let seconds = duration.num_microseconds().unwrap() as f64/1000000 as f64;
        // let throughput = index as f64/ seconds as f64;
        // println!("throughput: {}", throughput);.
        let shared_num = Arc::new(Mutex::new(0));
        let target_wasm_list = ["fb1.wasm", "fb2.wasm", "fb3.wasm", "fb4.wasm", "fb5.wasm", "fb6.wasm", "fb7.wasm", "fb8.wasm", "fb9.wasm", "fb10.wasm"];
        // let target_wasm_list = Vec::new();
        // for i in 0..index {
        //     target_wasm_list.push(format!("fb{}.wasm", i + 1));
        // }
        // for i in 0..target_wasm_list.len() {
        //     let fulture1 = mutil_send_request(response_string.clone(), target_wasm_list[i], Arc::clone(&shared_num));

        // }
        // let f1 = mutil_send_request(response_string.clone(), target_wasm_list[0], Arc::clone(&shared_num));
        // let f2 = mutil_send_request(response_string.clone(), target_wasm_list[1], Arc::clone(&shared_num));
        // let f3 = mutil_send_request(response_string.clone(), target_wasm_list[2], Arc::clone(&shared_num));
        println!("begin to exec task at {}", chrono::offset::Utc::now());
        // let flist: Future<Output = String> = Vec::new();
        // for i in 0..index {
        //     let f = mutil_send_request(response_string.clone(), &target_wasm_list[i].as_str(), i + 1);
        //     flist.push(f);
        // }
        let f1 = mutil_send_request(response_string.clone(), target_wasm_list[0], 1);
        let f2 = mutil_send_request(response_string.clone(), target_wasm_list[1], 2);
        let f3 = mutil_send_request(response_string.clone(), target_wasm_list[2], 3);
        let f4 = mutil_send_request(response_string.clone(), target_wasm_list[3], 4);
        let f5 = mutil_send_request(response_string.clone(), target_wasm_list[4], 5);
        let f6 = mutil_send_request(response_string.clone(), target_wasm_list[5], 6);
        let f7 = mutil_send_request(response_string.clone(), target_wasm_list[6], 7);
        let f8 = mutil_send_request(response_string.clone(), target_wasm_list[7], 8);
        // let f9 = mutil_send_request(response_string.clone(), target_wasm_list[8], 9);
        // let f10 = mutil_send_request(response_string.clone(), target_wasm_list[9], 10);

        // for f in flist {
        //     f.join();
        // }
        // join!(f1, f2, f3, f4, f5, f6, f7 ,f8, f9, f10);
        join!(f1, f2, f3, f4, f5, f6, f7, f8);
        
        // std::thread::sleep(std::time::Duration::from_secs(10));
        println!("all task compelted at {}", chrono::offset::Utc::now());
        let result:i32 = 5;
        return result;
    }
}

pub async fn mutil_send_request(input_string: &str, target_wasm: &str, count: i32) -> String{
    println!("Process response ");
    let start = chrono::offset::Utc::now();
    // let full_payload = "{\"source_channel\":\"func_a.wasm\",\"target_channel\":\"func_b.wasm\",\"payload\":\"".to_owned() +input_string+"\",\"start\":\"" +&start.to_string()+"\"}";
    
    let full_payload = "{\"source_channel\":\"func_a.wasm\",\"target_channel\":\"".to_owned() +
    &target_wasm.to_string() + 
    "\",\"payload\":\"" + 
    &input_string.to_string() + 
    "\",\"start\":\"" + 
    &start.to_string() + 
    "\"}";

    // let input_bytes = full_payload.as_bytes();
    let input_bytes = Arc::new(Mutex::new(full_payload.into_bytes()));
    let len = input_bytes.lock().unwrap().len() as i32;
    // let len = input_bytes.len() as i32;
    // let ptr = input_bytes.as_ptr();
    // let ptr_i32 = input_bytes.as_ptr() as i32;
    // let ptr_i32 = full_payload.as_ptr() as i32;
    //println!("input pointer {:?} ",ptr);
    //println!("input length {:?} ",len);

    unsafe {
        println!("Call external func at {}",chrono::offset::Utc::now());
        println!("To: fb{}.wasm. start transfer at {}",count, chrono::offset::Utc::now());
        // let response_length =cwasi_export::func_connect(ptr_i32,len);
        // println!("response from ext call received len {:?} at {:?}",response_length,chrono::offset::Utc::now());
        // let bytes = std::slice::from_raw_parts(ptr, response_length as usize);
        // println!("After bytes slice {}",chrono::offset::Utc::now());
        // let response = &std::str::from_utf8_unchecked(bytes);
        // println!("response string {:?} ",response);


        let cloned_input_bytes = Arc::clone(&input_bytes).lock().unwrap().clone();
        let input_bytes_ptr = cloned_input_bytes.as_ptr() as i32;
        let cloned_input_bytes_len = cloned_input_bytes.len();
        // let shared_number_clone = Arc::clone(&shared_num);
        // *shared_num.lock().unwrap() += 1;
        let connect_furture = async_func_connect(input_bytes_ptr, len, count).await;
        tokio::task::spawn(async move {
            // let mut num = shared_number_clone.lock().unwrap();
            
            // let cloned_input_bytes_ptr = cloned_input_bytes.as_ptr();

            // let cloned_input_bytes_ptr = Arc::as_ptr(&cloned_input_bytes);
            // let clone_num = num.clone();

            if let response_length = connect_furture {
                let cloned_input_bytes_ptr = cloned_input_bytes.as_ptr();
                println!("Response length {}",response_length);
                println!("fb{}.wasm, response from ext call received len {:?} at {:?}",count, response_length,chrono::offset::Utc::now());
                let bytes = std::slice::from_raw_parts(cloned_input_bytes_ptr, response_length as usize);
                println!("After bytes slice {}",chrono::offset::Utc::now());
                let response = &std::str::from_utf8_unchecked(bytes);
                println!("response string {:?} ",response);
            }
            
        }).await;
        // println!("func_a num {}", *shared_num.lock().unwrap());
        
        "finished".to_string()
    }
}

pub fn send_request(input_string: &str) -> String{
    println!("Process response ");
    let start = chrono::offset::Utc::now();
    //let full_payload = "{\"source_channel\":\"func_a.wasm\",\"target_channel\":\"func_b.wasm\",\"payload\":\"".to_owned() +input_string+"\",\"start\":\"" +&start.to_string()+"\"}";
    let full_payload = format!("task={} start {}",0, input_string);
    let input_bytes = full_payload.as_bytes();
    //let input_bytes = Arc::new(Mutex::new(full_payload.into_bytes()));
    //let len = input_bytes.lock().unwrap().len() as i32;
    let len = input_bytes.len() as i32;
    let ptr = input_bytes.as_ptr();
    //let ptr_i32 = input_bytes.as_ptr() as i32;
    let ptr_i32 = full_payload.as_ptr() as i32;
    //println!("input pointer {:?} ",ptr);
    //println!("input length {:?} ",len);

    unsafe {
        println!("Call external func at {}",chrono::offset::Utc::now());
        println!("start transfer at {}",chrono::offset::Utc::now());
        let response_length =cwasi_export::func_connect(ptr_i32,len, 0);
        println!("response from ext call received len {:?} at {:?}",response_length,chrono::offset::Utc::now());
        let bytes = std::slice::from_raw_parts(ptr, response_length as usize);
        println!("After bytes slice {}",chrono::offset::Utc::now());
        let response = &std::str::from_utf8_unchecked(bytes);
        // println!("response string {:?} ",response);

        // let cloned_input_bytes = Arc::clone(&input_bytes).lock().unwrap().clone();


        // let input_bytes_ptr = cloned_input_bytes.as_ptr() as i32;
        // let cloned_input_bytes_len = cloned_input_bytes.len();
        // tokio::task::spawn(async move {
        //     let cloned_input_bytes_ptr = cloned_input_bytes.as_ptr();
        //     if let response_length = async_func_connect(input_bytes_ptr,len).await{
        //         println!("Response length {}",response_length);
        //         println!("response from ext call received len {:?} at {:?}",response_length,chrono::offset::Utc::now());
        //         let bytes = std::slice::from_raw_parts(cloned_input_bytes_ptr, response_length as usize);
        //         println!("After bytes slice {}",chrono::offset::Utc::now());
        //         response = &std::str::from_utf8_unchecked(bytes);
        //     }
        //     println!("response string {:?} ",response);
        // });
        
        
        "finished".to_string()
    }

}
pub async fn async_func_connect(str_ptr: i32, str_len: i32, num: i32) -> i32 {
    unsafe { cwasi_export::func_connect(str_ptr, str_len, num) }
}
pub mod cwasi_export {
    #[link(wasm_import_module = "cwasi_export")]
    extern "C" {
        pub fn func_connect(ptr: i32, len: i32, num: i32) -> i32;
    }
}