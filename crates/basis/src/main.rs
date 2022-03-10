use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
use git2::{Commit, ObjectType, Repository, Sort};
use std::env;
use std::path::{Path, PathBuf};

// https://github.com/rust-lang/mdBook/pull/1506
fn main() -> std::io::Result<()> {
    git_snippet();
    Ok(())
}

fn git_snippet() {
    let files = [
        "crates/basis/src/main.rs",
        "crates/basis/Cargo.toml",
        ".github/workflows/ci.yml",
        "book.toml",
        "Cargo.toml",
        "Cargo.lock",
        "docs/README.md",
        "docs/SUMMARY.md",
        "README.md",
        "SUMMARY.md",
        "NOT_FOUND.md",
    ];

    for file in files {
        find_git_timestamp(file);
    }
}

fn find_git_timestamp(file_name: &str) {
    let mut timestamp = get_git_timestamp(file_name);

    // Fall back to build time.
    if timestamp == 0 {
        timestamp = Utc::now().timestamp();
        print!("[NOW] ");
    } else {
        print!("[GIT] ");
    }

    let china_time = get_china_time(timestamp);
    println!(
        "Commit {} at: {}",
        file_name,
        china_time.format("%Y-%m-%d %H:%M:%S"),
    );
}

fn get_git_timestamp(file_name: &str) -> i64 {
    let file_path = Path::new(file_name);
    let pwd_path = match env::current_dir() {
        Ok(pwd) => pwd,
        Err(_) => PathBuf::from("."),
    };
    let git_path = match find_git_path(pwd_path.as_path()) {
        Some(path) => path,
        None => return 0,
    };
    let repo = match Repository::open(git_path) {
        Ok(repo) => repo,
        Err(_) => return 0,
    };
    let head_commit = match get_head_commit(&repo) {
        Ok(commit) => commit,
        Err(_) => return 0,
    };
    let head_tree = match head_commit.tree() {
        Ok(tree) => tree,
        Err(_) => return 0,
    };
    let head_tree_entry = match head_tree.get_path(file_path) {
        Ok(entry) => entry,
        Err(_) => return 0,
    };
    let head_tree_entry_id = head_tree_entry.id();

    if head_tree_entry_id.is_zero() {
        println!("Couldn't find {} on head commit!", file_name);
        return 0;
    }

    let mut walker = match repo.revwalk() {
        Ok(walker) => walker,
        Err(_) => return 0,
    };
    match walker.push_head() {
        Ok(_) => {}
        Err(_) => return 0,
    };
    match walker.set_sorting(Sort::TIME) {
        Ok(_) => {}
        Err(_) => return 0,
    };
    let mut current_commit = head_commit;

    for commit_oid in walker {
        let oid = match commit_oid {
            Ok(oid) => oid,
            Err(_) => return 0,
        };
        let commit = match repo.find_commit(oid) {
            Ok(commit) => commit,
            Err(_) => return 0,
        };
        let tree = match commit.tree() {
            Ok(tree) => tree,
            Err(_) => return 0,
        };
        let tree_entry = match tree.get_path(file_path) {
            Ok(entry) => entry,
            Err(_) => continue,
        };

        // Find first object with same name but different SHA code.
        if tree_entry.id() != head_tree_entry_id {
            break;
        }

        current_commit = commit;
    }

    current_commit.time().seconds()
}

fn get_head_commit(repo: &Repository) -> Result<Commit<'_>, git2::Error> {
    let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    obj.into_commit()
        .map_err(|_| git2::Error::from_str("Couldn't find commit!"))
}

fn get_china_time(timestamp: i64) -> DateTime<FixedOffset> {
    let china_timezone = FixedOffset::east(8 * 3600);
    let utc_time = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(timestamp, 0), Utc);
    utc_time.with_timezone(&china_timezone)
}

fn find_git_path(path: &Path) -> Option<PathBuf> {
    let mut current_path = path;
    let mut git_dir = current_path.join(".git");
    let root = Path::new("/");

    while !git_dir.exists() {
        current_path = match current_path.parent() {
            Some(p) => p,
            None => return None,
        };

        if current_path == root {
            return None;
        }

        git_dir = current_path.join(".git");
    }

    git_dir.parent().map(|p| p.to_owned())
}
