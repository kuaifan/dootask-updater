use std::env;
use std::path::PathBuf;
use std::thread;
use std::time::{Duration, Instant};
use tauri::{Listener, Manager};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let args: Vec<String> = env::args().collect();

    tauri::Builder::default()
        .setup(move |app| {
            let window = app.get_webview_window("main").unwrap();

            // 如果有临时文件参数，读取其内容并作为查询参数
            if args.len() >= 2 {
                let tmp_file = PathBuf::from(&args[1]);

                let window_clone = window.clone();
                let tmp_file_clone = tmp_file.clone();
                let start_time = Instant::now();

                thread::spawn(move || {
                    loop {
                        let _ = window_clone.eval(&format!("document.body.appendChild(document.createTextNode('1111----'))"));

                        // 使用exists()方法替代metadata检查
                        if !tmp_file_clone.exists() {
                            // 安全地关闭窗口，忽略可能的错误
                            let _ = window_clone.eval(&format!(
                                "document.body.appendChild(document.createTextNode('参数长度：{}，文件不存在：{}----'))",
                                args.len(),
                                args[1]
                            ));
                            break;
                        }
                        let _ = window_clone.eval(&format!("document.body.appendChild(document.createTextNode('2222----'))"));

                        // 60秒超时保护
                        if start_time.elapsed().as_secs() > 60 {
                            let _ = window_clone.eval(&format!(
                                "document.body.appendChild(document.createTextNode('参数长度：{}，文件路径：{}，时间：{}秒----'))",
                                args.len(),
                                args[1],
                                start_time.elapsed().as_secs()
                            ));
                            break;
                        }
                        let _ = window_clone.eval(&format!("document.body.appendChild(document.createTextNode('3333----'))"));

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
