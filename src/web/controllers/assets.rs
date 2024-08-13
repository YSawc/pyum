use axum::{
    body::Body,
    extract::Path,
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
};

pub async fn get_image_asset(Path(path): Path<String>) -> impl IntoResponse {
    let image_path = format!("src/assets/images/{}", path);
    let file = match tokio::fs::File::open(image_path.to_owned()).await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    };
    let content_type = match mime_guess::from_path(&image_path).first_raw() {
        Some(mime) => mime,
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                "MIME Type couldn't be determined".to_string(),
            ))
        }
    };
    let stream = tokio_util::io::ReaderStream::new(file);
    let body = Body::from_stream(stream);

    let mut headermap = HeaderMap::new();
    headermap.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str(content_type).expect("header content type must be string."),
    );
    headermap.insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_str("inline").expect("header content of diposition must be string."),
    );

    Ok((headermap, body))
}
