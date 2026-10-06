use tauri::webview::NewWindowResponse;
use tauri::{AppHandle, Manager, Url, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_opener::OpenerExt;

const MAIN_WINDOW: &str = "main";
const START_URL: &str = "https://music.apple.com/";

/// Domains that stay inside the app. Everything else opens in the default browser.
/// Subdomains are included, so "apple.com" also covers sign-in (idmsa/appleid.apple.com).
const ALLOWED_DOMAINS: &[&str] = &["apple.com", "music.apple.com", "mzstatic.com"];

const INJECT_JS: &str = include_str!("../inject/inject.js");
const INJECT_CSS: &str = include_str!("../inject/inject.css");

fn is_allowed(url: &Url) -> bool {
    match url.scheme() {
        "https" => url.host_str().is_some_and(|host| {
            ALLOWED_DOMAINS
                .iter()
                .any(|d| host == *d || host.ends_with(&format!(".{d}")))
        }),
        // Internal pages the web player uses for frames and media.
        "about" | "blob" | "data" => true,
        _ => false,
    }
}

fn open_external(app: &AppHandle, url: &Url) {
    if let Err(err) = app.opener().open_url(url.as_str(), None::<&str>) {
        eprintln!("failed to open {url} in browser: {err}");
    }
}

/// Our CSS is handed to the injected script as a JSON string so it needs no escaping.
fn init_script() -> String {
    let css = serde_json::to_string(INJECT_CSS).expect("CSS serializes to a JSON string");
    format!("window.__WRAPPER_CSS__ = {css};\n{INJECT_JS}")
}

fn focus_main(app: &AppHandle) {
    if let Some(window) = app.get_webview_window(MAIN_WINDOW) {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // Must be registered first: a second launch just focuses the running app.
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            focus_main(app);
        }))
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .setup(|app| {
            let nav_handle = app.handle().clone();
            let popup_handle = app.handle().clone();

            WebviewWindowBuilder::new(
                app,
                MAIN_WINDOW,
                WebviewUrl::External(START_URL.parse().expect("valid start URL")),
            )
            .title("Music")
            .inner_size(1200.0, 800.0)
            .min_inner_size(480.0, 360.0)
            .initialization_script(init_script())
            .on_navigation(move |url| {
                if is_allowed(url) {
                    true
                } else {
                    open_external(&nav_handle, url);
                    false
                }
            })
            .on_new_window(move |url, _features| {
                if is_allowed(&url) {
                    // Apple's sign-in popup needs to stay in-app and share the session.
                    NewWindowResponse::Allow
                } else {
                    open_external(&popup_handle, &url);
                    NewWindowResponse::Deny
                }
            })
            .build()?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running the app");
}
