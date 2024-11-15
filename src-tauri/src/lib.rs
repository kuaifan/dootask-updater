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


            if args.len() >= 2 {
                let window_clone = window.clone();
                let file_clone = args[1].clone();
                let start_time = Instant::now();

                // 读取文件内容
                if let Ok(content) = fs::read_to_string(&file_clone) {
                    // 构建带查询参数的URL
                    if let Ok(base_url) = window_clone.url() {
                        let new_url = format!(
                            "{}?message={}",
                            base_url.as_str(),
                            urlencoding::encode(&content)
                        );
                        // 导航到新URL
                        let _ = window_clone.eval(&format!("window.location.href = '{}'", new_url));
                    }
                }

                thread::spawn(move || {
                    loop {
                        // 检查临时文件是否存在
                        if !fs::metadata(&file_clone).is_ok() {
                            let _ = window_clone.close();
                            break;
                        }
                        
                        // 60秒超时保护
                        if start_time.elapsed().as_secs() > 60 {
                            let _ = window_clone.close();
                            break;
                        }

                        thread::sleep(Duration::from_secs(1));
                    }
                });
            }

            // 监听自定义的页面加载完成事件
            let window_clone = window.clone();
            app.listen("page-loaded", move |_| {
                let _ = window_clone.show();
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
