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
                let tmp_file_clone = args[1].clone();
                let start_time = Instant::now();

                thread::spawn(move || {
                    loop {
                        let _ = window_clone.eval(&format!("document.body.appendChild(document.createTextNode('11----'))"));


                        if start_time.elapsed().as_secs() > 20 {
                            if !fs::metadata(&tmp_file_clone).is_ok() {
                                let _ = window_clone.eval(&format!("document.body.appendChild(document.createTextNode('文件不存在----'))"));
                            } else {
                                let _ = window_clone.eval(&format!("document.body.appendChild(document.createTextNode('文件还在----'))"));
                            }

                            if let Ok(content) = fs::read_to_string(&tmp_file_clone) {
                                if let Ok(base_url) = window_clone.url() {
                                    let _ = window_clone.eval(&format!("document.body.appendChild(document.createTextNode('文件读取成功, url: {}, 内容: {}----'))", base_url, content));
                                } else {
                                    let _ = window_clone.eval(&format!("document.body.appendChild(document.createTextNode('文件读取成功, url: 读取失败, 内容: {}----'))", content));  
                                }
                            } else {
                                let _ = window_clone.eval(&format!("document.body.appendChild(document.createTextNode('文件读取失败----'))"));
                            }
                        }


                        let _ = window_clone.eval(&format!("document.body.appendChild(document.createTextNode('22----'))"));

                        thread::sleep(Duration::from_secs(2));
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
