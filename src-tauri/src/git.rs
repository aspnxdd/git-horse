use git2::Repository;


pub struct Repo {
    pub repo: Repository,
}

impl Repo {
    pub fn new(repo: Repository) -> Self {
        Repo { repo }
    }
    pub fn open(path: &str) -> Result<Self, git2::Error> {
        let repo = Repository::open(path)?;
        Ok(Repo { repo })
    }
}
