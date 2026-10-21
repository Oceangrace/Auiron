use crate::core::icons::IconCache;
use std::sync::Arc;
use tauri::Manager;
use tauri::UriSchemeContext;
use tauri::UriSchemeResponder;
use tauri::Wry;
use tauri::http::HeaderValue;
use tauri::http::Request;
use tauri::http::Response;
use tauri::http::StatusCode;

pub(crate) struct IconProtocol;

impl IconProtocol {
    pub(crate) fn handle(
        context: UriSchemeContext<'_, Wry>,
        request: Request<Vec<u8>>,
        responder: UriSchemeResponder,
    ) {
        let cache = context
            .app_handle()
            .try_state::<Arc<IconCache>>()
            .map(|cache| cache.inner().clone());

        tauri::async_runtime::spawn_blocking(move || {
            responder.respond(Self::response(cache.as_deref(), &request));
        });
    }

    fn response(cache: Option<&IconCache>, request: &Request<Vec<u8>>) -> Response<Vec<u8>> {
        let mut response = Response::new(Vec::new());
        *response.status_mut() = StatusCode::NOT_FOUND;
        response
            .headers_mut()
            .insert("Cache-Control", HeaderValue::from_static("no-store"));
        response.headers_mut().insert(
            "X-Content-Type-Options",
            HeaderValue::from_static("nosniff"),
        );

        if request.method() != "GET" && request.method() != "HEAD" {
            *response.status_mut() = StatusCode::METHOD_NOT_ALLOWED;
            return response;
        }

        let Some(file) = request.uri().path().strip_prefix('/') else {
            return response;
        };
        let Some(bytes) = cache.and_then(|cache| cache.read(file)) else {
            return response;
        };
        *response.status_mut() = StatusCode::OK;
        response
            .headers_mut()
            .insert("Content-Type", HeaderValue::from_static("image/png"));

        if request.method() == "GET" {
            *response.body_mut() = bytes;
        }

        response
    }
}
