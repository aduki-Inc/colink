use actix_multipart::form::{
	tempfile::TempFile,
	MultipartForm,
};
use std::fmt;
// use actix_web::Error;
// use std::fs::{self, File};
use std::path::PathBuf;

// Custom file uploading error
#[derive(Debug)]
struct UploadError {
	message: String
}

impl fmt::Display for UploadError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f,"{}", self.message)
	}
}

impl std::error::Error for UploadError{}


#[derive(Debug, MultipartForm)]
struct UploadForm {
  #[multipart(rename = "file")]
  file: TempFile,
}


async fn upload_file(
	payload: MultipartForm<UploadForm>,
	name: &str,
	path_to: &str
) -> Result<String, UploadError> {

	const MAX_FILE_SIZE: u64 = 1024 * 1024 * 10; // 10 MB

	// reject malformed requests - If size is zero or less
	if payload.file.size <= 0 {
		return Err(UploadError{
			message: "The uploaded file size is zero bytes.".to_string()
		})
	} 
	// reject malformed requests - If size is greater  than 10Mbs
	if payload.file.size > (1024 * 1024 * 10) {
		return Err(UploadError{
			message: format!("The uploaded file is too large. Maximum size is {} bytes.", MAX_FILE_SIZE).to_string()
		})
	}
	

	let temp_file_path = payload.file.file.path();

	let original_filename = payload.file.file_name.clone().unwrap();
	
  let extension = original_filename.rsplit('.').next().unwrap();

  // Generate a new unique filename with the same extension
  let new_filename = format!("{}.{}", name, extension);

	// Create path to save file
	let mut path = PathBuf::from(path_to);
	path.push(&new_filename);

	// 	Create string path
	// let path_str = format!("{}/{}", path_to, new_filename);


	// if std::path::Path::exists(&path) {
	// 	match std::fs::remove_file(&path) {
	// 		Ok(_) => {

	// 		},
	// 		Err(_) => todo!(),
	// 	}
	// }


	// form.file.file.persist(path)?;

	// Save file and return path str
	// match payload.file.file.persist(path_str) {
  //   Ok(_persisted_file) => Ok(path.to_str().unwrap().to_string()),
  //   Err(_) => Err("Error has occurred during the file upload!")
	// };

	// match std::fs::rename(temp_file_path, path) {
	// 	Ok(_) => Ok(path.to_str().unwrap().to_string()),
	// 	Err(_) => Err(UploadError{
	// 		message: "Could not upload your file, Internal error occurred!.".to_string()
	// 	}),
	// }

	// std::fs::rename(temp_file_path, path.clone())?;
	// // Return uploaded file path
	// Ok(path.to_str().unwrap().to_string())

	match std::fs::rename(temp_file_path, path.clone()) {
    Ok(_) => Ok("this is a string".to_string()),
    Err(_) => Err(UploadError{
			message: "Could not upload your file, Internal error occurred!.".to_string()
		})
	}
}
