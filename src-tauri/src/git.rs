use crate::error::GitError;
use git2::{BranchType, Repository};

pub struct Repo {
    pub repo: Repository,
}

impl Repo {
    pub fn new(repo: Repository) -> Self {
        Repo { repo }
    }
    pub fn open(path: &str) -> Result<Self, GitError> {
        let repo = Repository::discover(path)?;
        Ok(Repo { repo })
    }
    pub fn get_current_branch_name(&self) -> Result<String, GitError> {
        let head = self.repo.head()?;
        let head_name = head.shorthand().unwrap();
        Ok(head_name.to_string())
    }
    pub fn get_repo_name(&self) -> Result<String, GitError> {
        let path_name = self.repo.path().to_str().unwrap().to_string();
        let repo_name: Vec<&str> = path_name.split("/").collect();
        return Ok(repo_name[repo_name.len() - 3].to_owned());
    }

    pub fn checkout_branch(&self, branch_name: &str) -> Result<(), GitError> {
        if let Ok(branch) = self.repo.find_branch(&branch_name, BranchType::Local) {
            if branch.get().is_branch() {
                let branch_ref = branch.get().name().unwrap();
                self.repo.set_head(&branch_ref)?;
                return Ok(());
            }
        }

        // Should create local branch of the remote branch
        if let Ok(branch) = self.repo.find_branch(&branch_name, BranchType::Remote) {
            if branch.get().is_remote() {
                let branch_ref = branch.get().name().unwrap();
                // let new_branch = repo.branch(&branch_ref, BranchType::Local)?;
                self.repo.set_head(&branch_ref)?;
                return Ok(());
            }
        }
        Err(GitError::GitCheckoutError)
    }
}
