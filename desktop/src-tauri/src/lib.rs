use tauri::{webview::NewWindowResponse, WebviewUrl, WebviewWindowBuilder};

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Normally the window opens the small launcher page (dist/index.html), which
            // checks the connection and then loads the live board. Setting CASEBOARD_URL
            // points the window somewhere else, e.g. http://localhost:3000 while developing.
            let start = match std::env::var("CASEBOARD_URL") {
                Ok(u) if !u.is_empty() => WebviewUrl::External(u.parse::<tauri::Url>()?),
                _ => WebviewUrl::App("index.html".into()),
            };

            WebviewWindowBuilder::new(app, "main", start)
                .title("Case Board")
                .inner_size(1440.0, 900.0)
                .min_inner_size(900.0, 600.0)
                .center()
                // Links that would open a new window ("Open", YouTube, Instagram...)
                // go to the user's own browser instead of a bare popup.
                .on_new_window(|url, _features| {
                    let _ = tauri_plugin_opener::open_url(url.as_str(), None::<&str>);
                    NewWindowResponse::Deny
                })
                .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Case Board");
}
