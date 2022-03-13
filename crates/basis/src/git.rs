use git2::{Commit, ObjectType, Repository};
use std::path::{Path, PathBuf};

pub trait GetHeadCommit {
    fn get_head_commit(&self) -> Result<Commit<'_>, git2::Error>;
}

impl GetHeadCommit for Repository {
    fn get_head_commit(&self) -> Result<Commit<'_>, git2::Error> {
        let obj = self.head()?.resolve()?.peel(ObjectType::Commit)?;
        obj.into_commit()
            .map_err(|_| git2::Error::from_str("Couldn't find commit!"))
    }
}

pub trait ToGitPath {
    fn to_git_path(&self) -> Option<PathBuf>;
}

impl ToGitPath for PathBuf {
    fn to_git_path(&self) -> Option<PathBuf> {
        let root = Path::new("/");
        let mut current_path = self.as_path();
        let mut git_dir = current_path.join(".git");

        while !git_dir.exists() {
            current_path = current_path.parent().unwrap_or(root);

            if current_path == root {
                return None;
            }

            git_dir = current_path.join(".git");
        }

        git_dir.parent().map(|p| p.to_owned())
    }
}
