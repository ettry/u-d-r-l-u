use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Write, BufReader};
use serde::Deserialize;
use std::process::Command;
use std::fs::File;
use serde_json::{Result as SerdeResult, Value, Error as SerdeError};
use thiserror::Error;
use std::env;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Event error:{0}")]
    EventError(String),
    #[error("Serde JSON error: {0}")]
    SerdeError(#[from] SerdeError),
    #[error("IO error: {0}")] IoError(#[from] std::io::Error),
}

#[derive(Deserialize)]
struct Config {
    A3_1:Vec<String>,
    A3_2:Vec<String>,
    A3_3:Vec<String>,
    A3_path:Vec<String>,
    
    B4_1:Vec<String>,
    B4_2:Vec<String>,
    B4_3:Vec<String>,
    B4_4:Vec<String>,
    B4_path:Vec<String>,

    C5_1:Vec<String>,
    C5_2:Vec<String>,
    C5_3:Vec<String>,
    C5_4:Vec<String>,
    C5_5:Vec<String>,
    C5_path:Vec<String>,
}

fn read_json<P: AsRef<std::path::Path>>(path: P) -> Result<Config, MyError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let config: Config = serde_json::from_reader(reader)?;
    Ok(config)
}

fn for_config(bv: Vec<String>, kl: &mut Vec<KeyCode>) {
    for asdd in bv.iter() {
        if asdd.to_lowercase() == "right" {
            kl.push(KeyCode::Right);
        } else if asdd.to_lowercase() == "down" {
            kl.push(KeyCode::Down);
        } else if asdd.to_lowercase() == "left" {
            kl.push(KeyCode::Left);
        } else {
            kl.push(KeyCode::Up);
        }
    }
}

fn d_3(a1: KeyCode, b1: KeyCode, c1: KeyCode, qwe: &mut Vec<KeyCode>, op1: &str) {
    if qwe.len() >= 3
        && qwe[qwe.len() - 3] == a1
        && qwe[qwe.len() - 2] == b1
        && qwe[qwe.len() - 1] == c1
    {
        let software_path = op1;
        if let Err(e) = Command::new(software_path).spawn() {
            eprintln!("Failed to start the software: {}", e);
        }
        // 清空按键序列
        qwe.clear();
        println!("清空输入记录");
    }
}


fn d_4(a2:KeyCode,b2:KeyCode,c2:KeyCode,d2:KeyCode,qwer: &mut Vec<KeyCode>,op2:&str) {
    if qwer.len() >= 4
        && qwer[qwer.len() - 4] == a2
        && qwer[qwer.len() - 3] == b2
        && qwer[qwer.len() - 2] == c2
        && qwer[qwer.len() - 1] == d2
    {
        let software_path = op2;
        if let Err(e) = Command::new(software_path).spawn() {
            eprintln!("Failed to start the software: {}", e);
        }
            // 清空按键序列
        qwer.clear();
        println!("清空输入记录");
    }

}
fn d_5(a3:KeyCode,b3:KeyCode,c3:KeyCode,d3:KeyCode,e5:KeyCode,qwert: &mut Vec<KeyCode>,op3:&str) {
    if qwert.len() >= 5
        && qwert[qwert.len() - 5] == a3
        && qwert[qwert.len() - 4] == b3
        && qwert[qwert.len() - 3] == c3
        && qwert[qwert.len() - 2] == d3
        && qwert[qwert.len() - 1] == e5
    {
        let software_path = op3;
        if let Err(e) = Command::new(software_path).spawn() {
            eprintln!("Failed to start the software: {}", e);
        }
            // 清空按键序列
        qwert.clear();
        println!("清空输入记录");
    }

}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let exe_path = env::current_exe()?;
    let mut json_path = exe_path.parent().unwrap().to_path_buf();
    json_path.push("user_date.json");
    let config = read_json(json_path)?;

    // 启用原始模式
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    // 用于记录按键顺序的队列
    let mut key_sequence = Vec::new();
    println!("tap键查看当前输入");
    println!("delete键重置输入");
    println!("esc键退出");

    let mut a_1:Vec<KeyCode>  = Vec::new();
    let mut a_2:Vec<KeyCode>  = Vec::new();
    let mut a_3:Vec<KeyCode>  = Vec::new();
    for_config(config.A3_1, &mut a_1);
    for_config(config.A3_2, &mut a_2);
    for_config(config.A3_3, &mut a_3);

    let mut b_1:Vec<KeyCode>  = Vec::new();
    let mut b_2:Vec<KeyCode>  = Vec::new();
    let mut b_3:Vec<KeyCode>  = Vec::new();
    let mut b_4:Vec<KeyCode>  = Vec::new();
    for_config(config.B4_1, &mut b_1);
    for_config(config.B4_2, &mut b_2);
    for_config(config.B4_3, &mut b_3);
    for_config(config.B4_4, &mut b_4);

    let mut c_1:Vec<KeyCode>  = Vec::new();
    let mut c_2:Vec<KeyCode>  = Vec::new();
    let mut c_3:Vec<KeyCode>  = Vec::new();
    let mut c_4:Vec<KeyCode>  = Vec::new();
    let mut c_5:Vec<KeyCode>  = Vec::new();
    for_config(config.C5_1, &mut c_1);
    for_config(config.C5_2, &mut c_2);
    for_config(config.C5_3, &mut c_3);
    for_config(config.C5_4, &mut c_4);
    for_config(config.C5_5, &mut c_5);


    loop {
        if event::poll(std::time::Duration::from_millis(1000))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Up => {
                        key_sequence.push(KeyCode::Up);
                        print!("UP ");
                        io::stdout().flush().unwrap();
                    }
                    KeyCode::Down => {
                        key_sequence.push(KeyCode::Down);
                        print!("Down ");
                        io::stdout().flush().unwrap();
                    }
                    KeyCode::Left => {
                        key_sequence.push(KeyCode::Left);
                        print!("Left ");
                        io::stdout().flush().unwrap();
                    }
                    KeyCode::Right => {
                        key_sequence.push(KeyCode::Right);
                        print!("Right ");
                        io::stdout().flush().unwrap();
                    }
                    KeyCode::Delete => {
                        key_sequence.clear();
                        println!("清空输入记录");
                    }
                    KeyCode::Esc => {
                        break;
                    }
                    _ => {}
                }

                // 检查按键序列是否为 UP 然后 DOWN
                for q in 0..a_1.len(){
                    d_3(a_1[q], a_2[q], a_3[q], &mut key_sequence, & config.A3_path[q]);
                }
                for w in  0..b_1.len(){
                    d_4(b_1[w],b_2[w],b_3[w],b_4[w],&mut key_sequence,& config.B4_path[w])
                }
                for e in 0..c_1.len(){
                    d_5(c_1[e],c_2[e],c_3[e],c_4[e],c_5[e],&mut key_sequence,& config.C5_path[e])
                }
                
            }
        }
    }

    // 恢复终端设置
    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}
