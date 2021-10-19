use git2;
use std::{env, path::Path};
use uuid::Uuid;

pub struct Repo {
	raw: git2::Repository,
	tree: RepoTree,
}

impl Repo {
	pub fn new_bare(repo_id: Uuid, owner_id: Uuid) -> git2::Repository {
		let path = Repo::path(repo_id, owner_id);
		git2::Repository::init_bare(&Path::new(&path)).unwrap()
	}

	pub fn open_bare(repo_id: Uuid, owner_id: Uuid) -> git2::Repository {
		let path = Repo::path(repo_id, owner_id);
		git2::Repository::open_bare(&Path::new(&path)).unwrap()
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

type Hash = String;

struct RepoTree {
}

struct TreeEntry<T>{
	hash: Hash,
	kind: ObjectType,
	mode: ObjectMode,
	name: String,
	object: T,
}


enum ObjectType{
	Blob,
	Tree,
}

enum ObjectMode {
	Normal,
	Executable,
	Symlink,
}

//struct Reference;

#[cfg(test)]
mod tests {
	#[test]
	fn create() {}
}
