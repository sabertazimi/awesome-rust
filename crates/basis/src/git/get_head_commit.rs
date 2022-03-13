use git2::{Commit, ObjectType, Repository};

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
