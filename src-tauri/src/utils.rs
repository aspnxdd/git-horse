use crate::git::Repo;

pub fn get_absolute_path_from_relative(repo: &Repo, relative_path: &str) -> String {
    let file_abs_path = format!(
        "{}{}",
        repo.repo.path().to_str().unwrap().replace("/.git", ""),
        relative_path
    );
    file_abs_path
}

pub fn get_origin_and_current_name_from_line(line: &str) -> (String, String) {
    let origin_name = line.split(" ").nth(2).unwrap().trim_start_matches("a/");
    let current_name = line.split(" ").nth(3).unwrap().trim_start_matches("b/");
    (origin_name.to_string(), current_name.to_string())
}
