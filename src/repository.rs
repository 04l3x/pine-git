use super::tree::Tree;
use error::Result;
use git2;
use std::{
	env,
	path::{Path, PathBuf},
};
use uuid::Uuid;

pub struct Repository;

impl Repository {
	pub fn get_tree(path: PathBuf) -> Result<Tree> {
		let repo = git2::Repository::open_bare(path).unwrap();
		let head = repo.head().unwrap();
		Tree::build(head)
	}

	pub fn init_bare<'r>(repo_id: Uuid, owner_id: Uuid) -> Result<()> {
		let path = Repository::path(repo_id, owner_id);
		match git2::Repository::init_bare(path) {
			Ok(_) => Ok(()),
			Err(e) => Err(Box::new(e)),
		}
	}

	pub fn open_with_uuid_info(repo_id: Uuid, owner_id: Uuid) -> git2::Repository {
		let path = Repository::path(repo_id, owner_id);
		git2::Repository::open_bare(&Path::new(&path)).unwrap()
	}

	pub fn open_with_path(path: PathBuf) -> git2::Repository {
		git2::Repository::open_bare(path).unwrap()
	}

	fn path(repo_id: Uuid, owner_id: Uuid) -> String {
		format!(
			"{}/{}/{}.git",
			env::var("GIT_ROOT_DIR").expect("not git root var"),
			owner_id.to_string(),
			repo_id.to_string(),
		)
	}
}
