use async_graphql::{Enum, SimpleObject};
use error::Result;
use git2::{ObjectType, TreeWalkMode, TreeWalkResult};

/// representation of tree of git objects as vec
#[derive(Clone, Debug, SimpleObject)]
pub struct Tree {
	objects: Vec<Object>,
}

impl Default for Tree {
	fn default() -> Self {
		Self { objects: vec![] }
	}
}

#[derive(Clone, Debug, SimpleObject)]
pub struct Object {
	pub path: String,
	pub kind: ObjectKind,
	pub name: String,
}

impl Object {
	fn new(path: String, kind: ObjectKind, name: String) -> Object {
		Object { path, kind, name }
	}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Enum)]
pub enum ObjectKind {
	Blob,
	Tree,
	Commit,
	Any,
	Tag,
}

impl From<ObjectType> for ObjectKind {
	fn from(value: ObjectType) -> Self {
		match value {
			ObjectType::Tree => ObjectKind::Tree,
			ObjectType::Blob => ObjectKind::Blob,
			ObjectType::Commit => ObjectKind::Commit,
			ObjectType::Any => ObjectKind::Any,
			ObjectType::Tag => ObjectKind::Tag,
		}
	}
}

impl Tree {
	fn add(&mut self, object: Object) {
		self.objects.push(object);
	}

	pub fn build(head: git2::Reference) -> Result<Tree> {
		let mut result = Tree::default();
		let tree = head.peel_to_tree();

		match tree {
			Ok(tree) => {
				let mut ct = 0;
				tree.walk(TreeWalkMode::PreOrder, |root, entry| {
					let obj = Object::new(
						root.to_string(),
						ObjectKind::from(entry.kind().expect("")),
						entry.name().expect("").to_string(),
					);
					result.add(obj);
					ct += 1;
					TreeWalkResult::Ok
				})
				.unwrap();
				Ok(result)
			}
			Err(e) => Err(Box::new(e)),
		}
	}
}
