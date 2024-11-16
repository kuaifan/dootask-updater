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
            let update_title = env::var("UPDATER_TITLE").unwrap_or_else(|_| String::from("默认标题"));

            if args.len() >= 2 {
                let window_clone = window.clone();
                let file_clone = args[1].clone();
                let start_time = Instant::now();

                thread::spawn(move || {
                    loop {
                        // 检查临时文件是否存在
                        if !fs::metadata(&file_clone).is_ok() {
                            let _ = window_clone.eval("typeof window.updateCompleted === 'function' && window.updateCompleted()");
                            thread::sleep(Duration::from_millis(500));
                            let _ = window_clone.close();
                            break;
                        }
                        
                        // 90秒超时保护
                        if start_time.elapsed().as_secs() > 90 {
                            let _ = window_clone.close();
                            break;
                        }

                        thread::sleep(Duration::from_secs(1));
                    }
                });
            }

            // 监听页面加载完成更新标题
            let window_clone = window.clone();
            let update_title_clone = update_title.clone();
            app.listen("page-loaded", move |_| {
                let _ = window_clone.eval(&format!(
                    "Promise.resolve().then(() => {{
                        if (typeof window.updateTitle === 'function') {{
                            return window.updateTitle('{}');
                        }}
                    }}).then(() => {{
                        window.appsEmit('title-updated');
                    }});",
                    update_title_clone
                ));
            });

            // 监听更新标题完成显示窗口
            let window_show = window.clone();
            app.listen("title-updated", move |_| {
                let _ = window_show.show();
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
