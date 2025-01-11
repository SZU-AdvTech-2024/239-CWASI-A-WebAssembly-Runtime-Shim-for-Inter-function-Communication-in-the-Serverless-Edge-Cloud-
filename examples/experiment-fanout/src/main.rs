use core::time;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::process::{Command, Output};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use std::{path, str};
use chrono::{format, Date, DateTime, TimeDelta, Utc};
use regex::Regex;

fn func_b(num: i32) -> Option<String> {
    println!("Starting func_b{}...", num);
    // Run the func_b container using ctr and capture the output
    let run_b_output: Output = Command::new("sudo")
        .args(&[
            "ctr",
            "-n", "mysp", "run", "--rm", "--runtime=io.containerd.cwasi.v1",
            "--annotation", "cwasi.secondary.function=true",
            "--net-host=true",
            &format!("docker.io/library/fb{}:latest", num), &format!("{}", rand::random::<u16>()),
            &format!("/fb{}.wasm", num),
        ])
        .output()
        .expect("Failed to run func_b");

    if run_b_output.status.success() {
        // println!("func_b executed successfully.");
        // return Some("func_b executed successfully.".to_string());
    } else {
        println!("func_b failed to execute.");
        return None;
    }

    // Convert output to a string and search for the relevant line
    let output_str = str::from_utf8(&run_b_output.stdout).expect("Failed to read output");
    if let Some(args_read_line) = output_str.lines().find(|line| line.contains("Received")) {
        // println!("Found func_b timestamp: {}", args_read_line);
        return Some(args_read_line.to_string().split("at ").last().unwrap().to_string());
    }

    None
}


fn compute_time() {
    let paths: [&str; 2] = ["/home/zhengyuanfeng/fb_output.txt", "/home/zhengyuanfeng/fa_output.txt"];
    let count = 200;

    let mut log_data = r#"

    "#;
    // 打开文件
    let mut fileb = File::open(paths[0]).unwrap();
    
    // 创建一个字符串来存储文件内容
    let mut contents = String::new();
    
    // 读取文件内容到字符串中
    fileb.read_to_string(&mut contents).unwrap();
    
    // 打印文件内容
    // println!("文件内容:\n{}", contents.as_str());
    log_data = contents.as_str();
    
    // 定义正则表达式，用于匹配时间戳
    let re = Regex::new(r"Received \d+ bytes at (.+?) UTC").unwrap();

    // 创建一个存储时间戳的向量
    let mut end_timestamps = Vec::new();

    // 遍历匹配的结果并提取时间戳
    for cap in re.captures_iter(log_data) {
        if let Some(timestamp) = cap.get(1) {
            end_timestamps.push(timestamp.as_str().to_string());
        }
    }

    // // 打印提取的时间戳
    // for timestamp in end_timestamps {
    //     println!("{}", timestamp);
    // }



    let mut log_data = r#"

    "#;

    let mut filea = File::open(paths[1]).unwrap();
    // 创建一个字符串来存储文件内容
    let mut contents = String::new();
        
    // 读取文件内容到字符串中
    filea.read_to_string(&mut contents).unwrap();
    
    // 打印文件内容
    // println!("文件内容:\n{}", contents.as_str());
    log_data = contents.as_str();
    
    // 定义正则表达式，用于匹配时间戳
    let re = Regex::new(r"start transfer at (.+?) UTC").unwrap();
    let re2 = Regex::new(r"end transfer at (.+?) UTC").unwrap();
    // 创建一个存储时间戳的向量
    let mut start_timestamps = Vec::new();
    let mut re_end_timestamps = Vec::new();
    // 遍历匹配的结果并提取时间戳
    for cap in re.captures_iter(log_data) {
        if let Some(timestamp) = cap.get(1) {
            start_timestamps.push(timestamp.as_str().to_string());
        }
    }
    

    for cap in re2.captures_iter(log_data) {
        if let Some(timestamp) = cap.get(1) {
            re_end_timestamps.push(timestamp.as_str().to_string());
        }
    }
    // // 打印提取的时间戳
    // for timestamp in start_timestamps {
    //     println!("{}", timestamp);
    // }

    let mut fanout_total_delta = 0 as f64;
    let mut fanin_total_delta = 0 as f64;
    for i in 0..count {
        start_timestamps[i].push_str(" UTC");
        end_timestamps[i].push_str(" UTC");   
        re_end_timestamps[i].push_str(" UTC");
        // println!("{}, {}", start_timestamps[i], end_timestamps[i]);
        let t1 = start_timestamps.get(i).unwrap().clone().parse::<DateTime<Utc>>().unwrap();
        let t2 = end_timestamps.get(i).unwrap().clone().parse::<DateTime<Utc>>().unwrap();
        let t3 = re_end_timestamps.get(i).unwrap().clone().parse::<DateTime<Utc>>().unwrap();
        let fanout_offset = t2 - t1;
        let fanin_offset = t3 - t2;
        println!("fanout_offset {}, {}", i, fanout_offset.num_nanoseconds().unwrap() as f64 / 1000000000 as f64);
        println!("fanin_offset {}, {}", i, fanin_offset.num_nanoseconds().unwrap() as f64 / 1000000000 as f64);
        fanout_total_delta += fanout_offset.num_nanoseconds().unwrap() as f64 / 1000000000 as f64;
        fanin_total_delta += fanin_offset.num_nanoseconds().unwrap() as f64 / 1000000000 as f64;
        
    }
    println!("fanout_total_delta: {}", fanout_total_delta);
    println!("average fanout delta: {}", fanout_total_delta / count as f64);
    println!("fanout_throughput: {}", count as f64 / fanout_total_delta);

    println!("=================================================================");

    println!("fanin_total_delta: {}", fanin_total_delta);
    println!("average fanin delta: {}", fanin_total_delta / count as f64);
    println!("fanin_throughput: {}", count as f64 / fanin_total_delta);


    let time1 = String::from("2024-12-06 08:52:18.595995801 UTC");
    
    let time2 = String::from("2024-12-06 08:52:19.351121331 UTC");

    let t1 = time1.parse::<DateTime<Utc>>().unwrap();
    let t2 = time2.parse::<DateTime<Utc>>().unwrap();

    println!("OFFSET: {}", t2 - t1)
}


fn main() {

    // let executions =  8;
    // let mut fb_thread_list: Vec<JoinHandle<Option<String>>> = Vec::new();
    // let mut fb_map = HashMap::new();
    // for i in 1..=executions {
    //     let fb_thread = thread::spawn(move || {
    //         func_b(i)
    //     });
    //     // fb_thread_list.push(fb_thread);
    //     fb_map.insert(i, fb_thread);
    // }
    // // println!("--------------------------------------------------------");
    // for (index, fb_thread) in fb_map {
    //     let result = fb_thread.join().unwrap().unwrap();
    //     println!("fb{}.wasm {}", index, result);
    // }

    // PT0.200928782S
    // 你根据前面的.wasm表示来匹配这些时间戳，并计算相对应的时间差多少秒，然后将所有差值向加起来
    compute_time();
}
