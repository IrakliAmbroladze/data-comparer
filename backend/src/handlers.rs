use crate::parsers::{csv_parser, excel_parser};
use axum::{extract::Multipart, http::StatusCode, Json};
use data_comparer_shared::Dataset;
use std::io::Cursor;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct UploadResponse {
    pub dataset: Dataset,
}

pub async fn upload_handler(mut multipart: Multipart) -> Result<Json<UploadResponse>, StatusCode> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("").to_string();

        if name == "file" {
            let filename = field.file_name().unwrap_or("").to_string();
            let dataset_name = field.name().unwrap_or("Dataset").to_string();
            let data = field.bytes().await.unwrap();

            let dataset = if filename.ends_with(".csv") {
                csv_parser::parse_csv(Cursor::new(data.to_vec()), dataset_name)
            } else if filename.ends_with(".xlsx") || filename.ends_with(".xlsm") {
                excel_parser::parse_excel(Cursor::new(data.to_vec()), dataset_name)
            } else {
                return Err(StatusCode::BAD_REQUEST);
            };

            match dataset {
                Ok(ds) => return Ok(Json(UploadResponse { dataset: ds })),
                Err(_) => return Err(StatusCode::BAD_REQUEST),
            }
        }
    }

    Err(StatusCode::BAD_REQUEST)
}

#[derive(serde::Deserialize)]
pub struct CompareRequest {
    pub dataset1: Dataset,
    pub dataset2: Dataset,
}

#[derive(serde::Serialize)]
pub struct CompareResponse {
    pub result: data_comparer_shared::ComparisonResult,
}

pub async fn compare_handler(Json(request): Json<CompareRequest>) -> Json<CompareResponse> {
    let result = crate::comparison::compare_datasets(request.dataset1, request.dataset2);
    Json(CompareResponse { result })
}
