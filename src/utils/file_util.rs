use actix_multipart::form::{
	tempfile::TempFile,
	MultipartForm,
};
use actix_web::Error;
// use std::fs::{self, File};
use std::path::PathBuf;


#[derive(MultipartForm)]
struct UploadForm {
  #[multipart(rename = "file")]
  file: TempFile,
}


async fn upload_file(
	MultipartForm(form): MultipartForm<UploadForm>,
	name: &str,
	path_to: &str
) -> Result<String, Error> {
	let original_filename = form.file.file_name.unwrap();
  let extension = original_filename.rsplit('.').next().unwrap();

  // Generate a new unique filename with the same extension
  let new_filename = format!("{}.{}", name, extension);

	// Create path to save file
	let mut path = PathBuf::from(path_to);
	path.push(&new_filename);

	// 	Create string path
	let path_str = format!("{}/{}", path_to, new_filename);


	if std::path::Path::exists(&path) {
		std::fs::remove_file(&path)?;
	}

	// form.file.file.persist(path)?;

	// Save file and return path str
	match form.file.file.persist(path_str) {
    Ok(_persisted_file) => Ok(path.to_str().unwrap().to_string()),
    Err(_) => Err("Error has occurred during the file upload!")
	};

	// Return uploaded file path
	Ok(path.to_str().unwrap().to_string())
}
