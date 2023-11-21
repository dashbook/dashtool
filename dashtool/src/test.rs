use git2::{Repository, RepositoryInitOptions};
use tempfile::TempDir;

pub fn repo_init() -> (TempDir, Repository) {
    let td = TempDir::new().unwrap();
    let mut opts = RepositoryInitOptions::new();
    opts.initial_head("main");
    let repo = Repository::init_opts(td.path(), &opts).expect("Failed to initialized git repo");
    {
        let mut config = repo.config().expect("Failed to read config of git repo");
        config.set_str("user.name", "name").unwrap();
        config.set_str("user.email", "email").unwrap();
        let mut index = repo.index().expect("Failed to get index of git repo");
        let id = index.write_tree().expect("Failed to write git tree");

        let tree = repo.find_tree(id).expect("Failed to get tree from repo");
        let sig = repo.signature().expect("Failed to get signature from repo");
        repo.commit(Some("HEAD"), &sig, &sig, "initial\n\nbody", &tree, &[])
            .expect("Failed to commit changes to git repo");
    }
    (td, repo)
}
