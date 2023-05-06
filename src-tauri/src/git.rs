use crate::error::GitError;
use git2::{BranchType, RemoteCallbacks, Repository};

pub struct Repo {
    pub repo: Repository,
}

pub fn get_remote_callbacks() -> RemoteCallbacks<'static> {
    let mut cb = RemoteCallbacks::new();
    let git_config = git2::Config::open_default().unwrap();
    let mut ch = git2_credentials::CredentialHandler::new(git_config);
    cb.credentials(move |url, username, allowed| ch.try_next_credential(url, username, allowed));
    return cb;
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
    pub fn get_repo_name(&self) -> Result<(String, String), GitError> {
        let path_name = self.repo.path().to_str().unwrap().to_string();
        let mut repo_name: Vec<&str> = path_name.split("/").collect();
        let repo_short_name = repo_name[repo_name.len() - 3].to_owned();
        repo_name.truncate(repo_name.len() - 2);
        let repo_path = repo_name.join("/");
        return Ok((repo_short_name.to_string(), repo_path.to_string()));
    }

    pub fn checkout_branch(&self, branch_name: &str) -> Result<(), GitError> {
        if let Ok(branch) = self.repo.find_branch(&branch_name, BranchType::Local) {
            if branch.get().is_branch() {
                let branch_ref = branch.get().name().unwrap();
                self.repo.set_head(&branch_ref)?;
                return Ok(());
            }
        }

        Err(GitError::GitCheckoutError)
    }
}
