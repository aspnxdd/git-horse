use crate::error::SledError;
use serde::Serialize;
use sled;

pub struct Db {
    db: sled::Db,
}
#[derive(Serialize)]
pub struct Repo {
    name: String,
    path: String,
}

const TMP_PATH: &str = "tmp/db";
const LAST_OPENED_REPO: &str = "last_opened_repo";
const THEME: &str = "theme";

impl Db {
    pub fn new() -> Result<Self, SledError> {
        let dir = std::env::current_dir().unwrap();
        let dir = dir.to_str().unwrap();
        let mut dir: Vec<&str> = dir.split("\\").collect();
        dir.pop().unwrap();
        let dir = dir.join("/");
        let dir = format!("{}/{}", dir, TMP_PATH);
        let db = sled::open(dir)?;
        Ok(Db { db })
    }
    pub fn write_last_opened_repo(&self, repo: &str) -> Result<(), SledError> {
        let key = LAST_OPENED_REPO;
        self.db.insert(key, repo)?;
        Ok(())
    }
    pub fn read_last_opened_repo(&self) -> Result<String, SledError> {
        let key = LAST_OPENED_REPO;
        let value = self.get(key)?;
        Ok(value)
    }
    pub fn write_theme(&self, new_theme: &str) -> Result<(), SledError> {
        self.db.insert(THEME, new_theme)?;
        Ok(())
    }
    pub fn read_theme(&self) -> Result<String, SledError> {
        let value = self.get(THEME)?;
        Ok(value)
    }

    pub fn get(&self, key: &str) -> Result<String, SledError> {
        if let Some(val) = self.db.get(key)? {
            let res = String::from_utf8(val.to_vec()).unwrap();
            return Ok(res);
        }
        return Err(SledError::SledError(
            "Can't open key from store".to_string(),
        ));
    }

    pub fn get_all(&self) -> Result<Vec<Repo>, SledError> {
        let res = self
            .db
            .iter()
            .keys()
            .filter_map(|k| {
                let repo_name = String::from_utf8(k.unwrap().to_vec()).unwrap();
                if (repo_name == LAST_OPENED_REPO) || (repo_name == THEME) {
                    return None;
                }
                let repo_path = self.get(&repo_name).unwrap();
                Some(Repo {
                    name: repo_name,
                    path: repo_path,
                })
            })
            .collect();
        Ok(res)
    }

    pub fn insert(&self, key: &str, value: &str) -> Result<(), SledError> {
        self.db.insert(key, value)?;
        Ok(())
    }

    pub fn remove(&self, key: &str) -> Result<(), SledError> {
        self.db.remove(key)?;
        Ok(())
    }

    pub fn clear(&self) -> Result<(), SledError> {
        self.db.clear()?;
        Ok(())
    }
}
