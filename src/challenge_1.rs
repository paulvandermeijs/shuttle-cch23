use std::error::Error;

use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Router};

pub fn router() -> Router {
    let router = Router::new().route("/*path", get(handler));

    router
}

async fn handler(Path(path): Path<String>) -> Result<impl IntoResponse, StatusCode> {
    Ok(cube_the_bits(path)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .to_string())
}

fn cube_the_bits(path: String) -> Result<u32, Box<dyn Error>> {
    let mut ids = path.split("/").collect::<Vec<&str>>();

    ids.truncate(20);

    let result = ids.iter().try_fold(0, |r, c| {
        let value = c.parse::<u32>()?;

        Ok::<u32, Box<dyn Error>>(r ^ value)
    })?;

    let result = result.pow(3);

    Ok(result)
}
