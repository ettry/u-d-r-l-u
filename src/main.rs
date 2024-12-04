use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{arch::x86_64, io::{self, BufReader, Write}};
use serde::Deserialize;
use std::process::Command;
use std::fs::File;
use serde_json::{Result as SerdeResult, Value, Error as SerdeError};
use thiserror::Error;
use std::env;
use open::that;

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
    a3_1:Vec<String>,
    a3_2:Vec<String>,
    a3_3:Vec<String>,
    a3_path:Vec<String>,
    
    b4_1:Vec<String>,
    b4_2:Vec<String>,
    b4_3:Vec<String>,
    b4_4:Vec<String>,
    b4_path:Vec<String>,

    c5_1:Vec<String>,
    c5_2:Vec<String>,
    c5_3:Vec<String>,
    c5_4:Vec<String>,
    c5_5:Vec<String>,
    c5_path:Vec<String>,

    d_file1_json:Vec<String>,
    d_file2_json:Vec<String>,
    d_file3_json:Vec<String>,
    d_file4_json:Vec<String>,
    d_file5_json:Vec<String>,
    d_file6_json:Vec<String>,
    d_file_path:Vec<String>,

    a3_name:Vec<String>,
    b4_name:Vec<String>,
    c5_name:Vec<String>,
    d_file_name:Vec<String>
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

fn if_Keycode(key_code:&str){
    if key_code.to_lowercase() == "right"{
        print!("➡️ ");
    }else if key_code.to_lowercase() == "down"{
        print!("⬇️ ")
    }else if  key_code.to_lowercase() =="left"{
        print!("⬅️ ")
    }else {
        print!("⬆️ ")
    }
}

fn d_3(a1: KeyCode, b1: KeyCode, c1: KeyCode, qwe: &mut Vec<KeyCode>, op1: &str,gh: &str) {
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
        //println!("清空输入记录");
        println!("已打开{}",gh)
    }
}


fn d_4(a2:KeyCode,b2:KeyCode,c2:KeyCode,d2:KeyCode,qwer: &mut Vec<KeyCode>,op2:&str,ghj:&str) {
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
        //println!("清空输入记录");
        println!("已打开{}",ghj)
    }

}
fn d_5(a3:KeyCode,b3:KeyCode,c3:KeyCode,d3:KeyCode,e3:KeyCode,qwert: &mut Vec<KeyCode>,op3:&str,ghjk:&str) {
    if qwert.len() >= 5
        && qwert[qwert.len() - 5] == a3
        && qwert[qwert.len() - 4] == b3
        && qwert[qwert.len() - 3] == c3
        && qwert[qwert.len() - 2] == d3
        && qwert[qwert.len() - 1] == e3
    {
        let software_path = op3;
        if let Err(e) = Command::new(software_path).spawn() {
            eprintln!("Failed to start the software: {}", e);
        }
            // 清空按键序列
        qwert.clear();
        //println!("清空输入记录");
        println!("已打开{}",ghjk)
    }

}

fn file_6(a4:KeyCode,b4:KeyCode,c4:KeyCode,d4:KeyCode,e4:KeyCode,f4:KeyCode,qwerty:&mut Vec<KeyCode>,op4:&str,ghjkl:&str ){
    if qwerty.len() >= 6
        && qwerty[qwerty.len() - 6] == a4
        && qwerty[qwerty.len() - 5] == b4
        && qwerty[qwerty.len() - 4] == c4
        && qwerty[qwerty.len() - 3] == d4
        && qwerty[qwerty.len() - 2] == e4
        && qwerty[qwerty.len() - 1] == f4
    {
        let software_path = op4;
        if let Err(e) =that(software_path) {
            eprintln!("Failed to start the software: {}", e);
        }
            // 清空按键序列
        qwerty.clear();
        //println!("清空输入记录");
        println!("已打开{}",ghjkl)
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    //定义json文件位置
    let exe_path = env::current_exe()?;
    let mut json_path = exe_path.parent().unwrap().to_path_buf();
    json_path.push("user_date.json");
    let config = read_json(json_path)?;

    // 启用原始模式
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    
    let mut key_sequence = Vec::new();
    println!("tap键查看当前输入");
    println!("delete键重置输入");
    println!("esc键退出");
    //打印文件配置
    for z in 0..config.a3_name.len(){
        if_Keycode(& config.a3_1[z]);
        if_Keycode(& config.a3_2[z]);
        if_Keycode(& config.a3_3[z]);
        println!("{}",config.a3_name[z]);
    }

    for x in 0..config.b4_name.len(){
       if_Keycode(&config.b4_1[x]);
       if_Keycode(&config.b4_2[x]);
       if_Keycode(&config.b4_3[x]);
       if_Keycode(&config.b4_4[x]);
       println!("{}",config.b4_name[x]);
    }

    for c in 0..config.c5_name.len(){
        if_Keycode(&config.c5_1[c]);
        if_Keycode(&config.c5_2[c]);
        if_Keycode(&config.c5_3[c]);
        if_Keycode(&config.c5_4[c]);
        if_Keycode(&config.c5_5[c]);
        println!("{}",config.c5_name[c]);
    }

    for v in 0..config.d_file_name.len(){
        if_Keycode(&config.d_file1_json[v]);
        if_Keycode(&config.d_file2_json[v]);
        if_Keycode(&config.d_file3_json[v]);
        if_Keycode(&config.d_file4_json[v]);
        if_Keycode(&config.d_file5_json[v]);
        if_Keycode(&config.d_file6_json[v]);
        println!("{}",config.d_file_name[v]);
    } 

    // 用于记录按键顺序的队列
    let mut a_1:Vec<KeyCode>  = Vec::new();
    let mut a_2:Vec<KeyCode>  = Vec::new();
    let mut a_3:Vec<KeyCode>  = Vec::new();
    for_config(config.a3_1, &mut a_1);
    for_config(config.a3_2, &mut a_2);
    for_config(config.a3_3, &mut a_3);

    let mut b_1:Vec<KeyCode>  = Vec::new();
    let mut b_2:Vec<KeyCode>  = Vec::new();
    let mut b_3:Vec<KeyCode>  = Vec::new();
    let mut b_4:Vec<KeyCode>  = Vec::new();
    for_config(config.b4_1, &mut b_1);
    for_config(config.b4_2, &mut b_2);
    for_config(config.b4_3, &mut b_3);
    for_config(config.b4_4, &mut b_4);

    let mut c_1:Vec<KeyCode>  = Vec::new();
    let mut c_2:Vec<KeyCode>  = Vec::new();
    let mut c_3:Vec<KeyCode>  = Vec::new();
    let mut c_4:Vec<KeyCode>  = Vec::new();
    let mut c_5:Vec<KeyCode>  = Vec::new();
    for_config(config.c5_1, &mut c_1);
    for_config(config.c5_2, &mut c_2);
    for_config(config.c5_3, &mut c_3);
    for_config(config.c5_4, &mut c_4);
    for_config(config.c5_5, &mut c_5);

    let mut d_file1:Vec<KeyCode> = Vec::new();
    let mut d_file2:Vec<KeyCode> = Vec::new();
    let mut d_file3:Vec<KeyCode> = Vec::new();
    let mut d_file4:Vec<KeyCode> = Vec::new();
    let mut d_file5:Vec<KeyCode> = Vec::new();
    let mut d_file6:Vec<KeyCode> = Vec::new();
    for_config(config.d_file1_json,&mut d_file1);
    for_config(config.d_file2_json,&mut d_file2);
    for_config(config.d_file3_json,&mut d_file3);
    for_config(config.d_file4_json,&mut d_file4);
    for_config(config.d_file5_json,&mut d_file5);
    for_config(config.d_file6_json,&mut d_file6);





    loop {
        if event::poll(std::time::Duration::from_millis(1000))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Up => {
                        key_sequence.push(KeyCode::Up);
                        print!("⬆️ ");
                        io::stdout().flush().unwrap();
                    }
                    KeyCode::Down => {
                        key_sequence.push(KeyCode::Down);
                        print!("⬇️ ");
                        io::stdout().flush().unwrap();
                    }
                    KeyCode::Left => {
                        key_sequence.push(KeyCode::Left);
                        print!("⬅️ ");
                        io::stdout().flush().unwrap();
                    }
                    KeyCode::Right => {
                        key_sequence.push(KeyCode::Right);
                        print!("➡️ ");
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
                    d_3(a_1[q], a_2[q], a_3[q], &mut key_sequence, & config.a3_path[q],&config.a3_name[q]);
                }
                for w in  0..b_1.len(){
                    d_4(b_1[w],b_2[w],b_3[w],b_4[w],&mut key_sequence,& config.b4_path[w],&config.b4_name[w])
                }
                for e in 0..c_1.len(){
                    d_5(c_1[e],c_2[e],c_3[e],c_4[e],c_5[e],&mut key_sequence,& config.c5_path[e],&config.c5_name[e])
                }
                for t in 0..d_file1.len(){
                    file_6(d_file1[t],d_file2[t],d_file3[t],d_file4[t],d_file5[t],d_file6[t],&mut key_sequence,&config.d_file_path[t],&config.d_file_name[t])
                }
                
            }
        }
    }

    // 恢复终端设置
    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}
