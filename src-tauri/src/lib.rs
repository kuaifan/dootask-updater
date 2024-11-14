use std::env;
use std::fs;
use std::thread;
use std::time::{Duration, Instant};
use tauri::{Listener, Manager};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let args: Vec<String> = env::args().collect();
    
    tauri::Builder::default()
        .setup(move |app| {
            let window = app.get_webview_window("main").unwrap();
            
            // 先隐藏窗口
            window.hide().unwrap();
            
            // 监听自定义的页面加载完成事件
            let window_clone = window.clone();
            app.listen("page-loaded", move |_| {
                window_clone.show().unwrap();
            });
            
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
