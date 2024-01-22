use actix_multipart::Multipart;
use actix_web::{Error, HttpRequest};
use futures_util::stream::StreamExt;
use std::fs::{self, File};
use std::io::{copy, Cursor};
use std::path::PathBuf;

async fn upload_file(
	req: HttpRequest,
	mut payload: Multipart,
	name: &str,
	path_to: &str,
) -> Result<String, Error> {
	let field = match payload.next().await {
		Some(Ok(field)) => field,
		_ => return Err(Error::from("No file found in the payload!")),
	};

	let filename_ext = field
		.content_disposition()
		.get_filename()
		.unwrap_or("unknown.ext")
		.to_string();

	// Define the new filename without changing extension
	let new_filename = format!("{}.{}", name, filename_ext);

	// Create path to save file
	let mut file_path = PathBuf::from(path_to);
	file_path.push(&new_filename);

	// Create directory if it doesn't exist
	if let Some(parent) = file_path.parent() {
		if !parent.exists() {
			fs::create_dir_all(parent)?;
		}
	}

	// If file with the same name exists, remove it
	if file_path.exists() {
		fs::remove_file(&file_path)?;
	}

	// Copy file data to the new location
	let mut out_file = File::create(&file_path)?;
	copy(&mut field.into_read(), &mut out_file)?;

	// Return uploaded file path
	Ok(file_path.to_str().unwrap().to_string())
}