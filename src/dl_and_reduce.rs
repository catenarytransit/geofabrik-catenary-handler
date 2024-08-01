use geofabrik_catenary_handler::{download_and_filter_all, SOURCES_TO_DOWNLOAD};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    download_and_filter_all(
        "temp_pbf_path/",
        "poly_folder_path/",
        "filtered_folder_path/",
    )
    .await;

    Ok(())
}
