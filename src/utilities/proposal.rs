use git2::{Repository, RepositoryInitOptions, Signature, FetchOptions, RemoteCallbacks, Cred};
use std::fs::File;
// use std::prelude::*;
use std::fmt;
use std::io::Write;
use crate::middlewares::auth::auth_middleware::UserClaims;
use std::path::PathBuf;
use tempdir::TempDir;


// Custom file uploading error
#[derive(Debug)]
pub struct RepoError {
	pub message: String
}

impl fmt::Display for RepoError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f,"{}", self.message)
	}
}

impl std::error::Error for RepoError{}

async fn initialize_proposal(
  name: &str,
  user: &UserClaims,
  path: PathBuf
) -> Result<String, RepoError> {
  let path_str = format!("{}/proposal.git", name).to_string();
  let proposal_path = path.join(&path_str);

  // Create a temporary dir
  let temp_dir = match TempDir::new("proposal") {
    Ok(created_dir) => created_dir,
    Err(_) => {
      return Err(RepoError{
			  message: "Could not initialize the process, Error occurred in the server!".to_string()
		  })
    }
  };


  let repo = match Repository::init_opts(temp_dir, &RepositoryInitOptions::new()) {
    Ok(repo) => repo,
    Err(_) => {
      return Err(RepoError{
			  message: "Could not initialize project, Error occurred in the server!".to_string()
		  })
    }
  };

  // Create a README.md file inside the temp_dir
  let file_path = temp_dir.path().join("README.md");
  let temp_file = match File::create(file_path) {
    Ok(created_file) => created_file,
    Err(_) =>  {
      return Err(RepoError{
			  message: "Something went wrong, Error occurred in the server!".to_string()
		  })
    },
  };

  // Write contents to the README.md file
  if let Err(e) = writeln!(&mut temp_file, "{}", readme_content) {
    return Err(RepoError{
			message: "Could not create initial file!".to_string()
		})
  }

  // Now adding README.md to the repository index
  let mut index = match repo.index() {
    Ok(index) => index,
    Err(_) => todo!()
  };

  index.add_path(&file_path).unwrap();
  index.write().unwrap();

  //Commit changes
  let tree_id = index.write_tree().unwrap();
  let tree = match repo.find_tree(tree_id) {
    Ok(tree) => tree,
    Err(_) => {
      return Err(RepoError{
			  message: "Something went wrong, Could not add the first commit!".to_string()
		  })
    },
  };

  let author = match Signature::now(&user.username, &user.email) {
    Ok(author) => author,
    Err(_) => {
      return Err(RepoError{
			  message: "Something went wrong!".to_string()
		  })
    },
  };

  let committer = match Signature::now(&user.username, &user.email) {
    Ok(committer) => committer,
    Err(_) => {
      return Err(RepoError{
			  message: "Something went wrong!".to_string()
		  })
    },
  };

  repo.commit(
    Some("HEAD"),
    &author,
    &committer,
    "Initial commit",
    &tree,
    &[]
  );

  // Bare clone (--bare) the repo to a new path
  let bare_repo = match Repository::init_bare(proposal_path) {
    Ok(bare_repo) => bare_repo,
    Err(_) => {
      return Err(RepoError{
			  message: "Something went wrong, Error occurred in the server!".to_string()
		  })
    },
  };

  bare_repo.remote("origin", temp_dir).unwrap();

  let mut fetch_options = FetchOptions::new();
  let mut callbacks = RemoteCallbacks::new();

  callbacks.credentials(|_,_,_| {
    Cred::default()
  });

  fetch_options.remote_callbacks(callbacks);


  let mut remote = bare_repo.find_remote("origin").unwrap();

  remote.fetch(&["refs/heads/*:refs/heads/*"], Some(&mut fetch_options), None).unwrap();


  // Drop/delete temporary file and directory
  drop(temp_file);
  temp_dir.close()?;

  Ok(proposal_path.to_str())
}