use std::env;
use std::fs;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use tauri::Manager;
use serde_json;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let args: Vec<String> = env::args().collect();

    tauri::Builder::default()
        .setup(move |app| {
            let window = app.get_webview_window("main").unwrap();

            // 将 args 转换为 JSON 字符串
            let args_json = serde_json::to_string(&args).unwrap_or_default();
            let _ = window.eval(&format!("document.body.appendChild('{}')", args_json));
            
            // 如果有临时文件参数，读取其内容并作为查询参数
            if args.len() == 2 {
                let tmp_file = PathBuf::from(&args[1]);

                // 读取文件内容
                if let Ok(content) = fs::read_to_string(&tmp_file) {
                    let _ = window.eval(&format!("document.body.appendChild('文件内容：{}')", content));
                }

                let tmp_file_clone = tmp_file.clone();

                thread::spawn(move || {
                    loop {
                        if !tmp_file_clone.exists() {
                            let _ = window.eval(&format!("document.body.appendChild('文件不存在')"));
                        } else {
                            let _ = window.eval(&format!("document.body.appendChild('文件存在')"));
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
