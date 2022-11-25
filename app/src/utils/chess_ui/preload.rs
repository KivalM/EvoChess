use crate::utils::image_future::ImageFuture;
use std::collections::HashMap;
use wasm_bindgen::JsValue;
use web_sys::HtmlImageElement;

pub async fn preload_pieces(theme: &str) -> Result<HashMap<String, HtmlImageElement>, JsValue> {
    let pieces = "rRbBnNqQkKpP";

    let mut map = HashMap::new();
    for piece in pieces.chars() {
        let theme = theme.clone();
        let url = format!("/assets/chess/{theme}/{piece}.svg");
        let img = preload_img(&url).await?;
        map.insert(piece.to_string(), img);
    }

    Ok(map)
}

pub async fn preload_img(url: &str) -> Result<HtmlImageElement, String> {
    ImageFuture::new(url)
        .await
        .map_err(|_| "Failed to load image".to_string())
}
