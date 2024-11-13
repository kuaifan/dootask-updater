use std::env;
use std::fs;
use std::thread;
use std::time::{Duration, Instant};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let args: Vec<String> = env::args().collect();
    
    tauri::Builder::default()
        .setup(move |app| {
            let window = app.get_webview_window("main").unwrap();
            let start_time = Instant::now();
            
            // 只有在提供参数时才进行文件检测
            if args.len() == 2 {
                let tmp_file = args[1].clone();
                
                thread::spawn(move || {
                    loop {
                        // 检查临时文件是否存在
                        if !fs::metadata(&tmp_file).is_ok() {
                            window.close().unwrap();
                            break;
                        }
                        
                        // 60秒超时保护
                        if start_time.elapsed().as_secs() > 60 {
                            window.close().unwrap();
                            break;
                        }
                        
                        thread::sleep(Duration::from_secs(1));
                    }
                });
            }
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
