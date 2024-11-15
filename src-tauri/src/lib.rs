use std::env;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use tauri::{Listener, Manager};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let args: Vec<String> = env::args().collect();

    tauri::Builder::default()
        .setup(move |app| {
            let window = app.get_webview_window("main").unwrap();

            let window_c = window.clone();
            let aaa = args.clone();

            thread::spawn(move || {
                loop {
                    let bbb = aaa.clone();
                    let _ = window_c.eval(&format!(
                        "document.body.appendChild(document.createTextNode('参数长度：{}，内容：{}----'))",
                        bbb.len(),
                        bbb.join(", ")
                    ));
                    for arg in bbb {
                        let tmp_file = PathBuf::from(&arg);
                        let _ = window_c.eval(&format!(
                            "document.body.appendChild(document.createTextNode('文件：{}，是否存在：{}----'))",
                            arg,
                            tmp_file.exists()
                        ));
                    }

                    thread::sleep(Duration::from_secs(1));
                }
            });


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
