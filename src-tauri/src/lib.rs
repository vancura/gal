mod photos;
mod thumbs;

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use tauri::http::{Response, StatusCode};
use tauri::{Manager, State};

#[derive(Clone)]
struct Library {
    by_id: Arc<Mutex<HashMap<String, PathBuf>>>,
    cache_dir: PathBuf,
}

#[tauri::command]
async fn list_photos(lib: State<'_, Library>) -> Result<Vec<photos::Photo>, String> {
    let lib = lib.inner().clone();
    tauri::async_runtime::spawn_blocking(move || {
        let listed = photos::list(&photos::library_dir(), &lib.cache_dir.join("dims.json"))?;
        let mut by_id = lib.by_id.lock().map_err(|e| e.to_string())?;
        by_id.clear();
        by_id.extend(listed.iter().map(|p| (p.id.clone(), p.path.clone())));
        Ok(listed)
    })
    .await
    .map_err(|e| e.to_string())?
}

/// `gal://localhost/thumb/<id>` and `gal://localhost/full/<id>`.
fn serve(lib: &Library, uri_path: &str) -> Response<Vec<u8>> {
    let mut parts = uri_path.trim_start_matches('/').splitn(2, '/');
    let kind = parts.next().unwrap_or("");
    let id = parts.next().unwrap_or("");
    let src = lib.by_id.lock().ok().and_then(|m| m.get(id).cloned());

    let bytes = match (kind, src) {
        // ponytail: whole file in memory, no Range support. Add it when videos or huge scans show up.
        ("full", Some(src)) => std::fs::read(&src).map_err(|e| e.to_string()),
        ("thumb", Some(src)) => thumbs::ensure(&src, &lib.cache_dir.join("thumbs"), id)
            .and_then(|p| std::fs::read(p).map_err(|e| e.to_string())),
        _ => Err(format!("unknown photo {uri_path}")),
    };

    match bytes {
        Ok(body) => Response::builder()
            .header("Content-Type", "image/jpeg")
            .header("Cache-Control", "max-age=31536000, immutable")
            .body(body),
        Err(msg) => Response::builder().status(StatusCode::NOT_FOUND).body(msg.into_bytes()),
    }
    .expect("static response headers are valid")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let cache_dir = app.path().app_cache_dir()?;
            app.manage(Library { by_id: Arc::default(), cache_dir });
            Ok(())
        })
        .register_asynchronous_uri_scheme_protocol("gal", |ctx, request, responder| {
            let lib = ctx.app_handle().state::<Library>().inner().clone();
            let path = request.uri().path().to_string();
            tauri::async_runtime::spawn_blocking(move || responder.respond(serve(&lib, &path)));
        })
        .invoke_handler(tauri::generate_handler![list_photos])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
