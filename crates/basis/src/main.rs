use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
use git2::{Commit, Error, ObjectType, Repository, Sort};
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
    let mut timestamp = match get_git_timestamp(file_name) {
        Ok(timestamp) => timestamp,
        Err(_) => 0,
    };

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

fn get_git_timestamp(file_name: &str) -> Result<i64, Error> {
    let file_path = Path::new(file_name);
    let pwd_path = env::current_dir().unwrap_or(PathBuf::from("."));
    let git_path = match find_git_path(pwd_path.as_path()) {
        Some(git_path) => git_path,
        None => return Err(Error::from_str("Not a git repository")),
    };
    let repo = Repository::open(git_path)?;
    let head_commit = get_head_commit(&repo)?;
    let head_entry = head_commit.tree()?.get_path(file_path)?;
    let head_entry_id = head_entry.id();

    if head_entry_id.is_zero() {
        println!("Couldn't find {} on head commit!", file_name);
        return Err(Error::from_str("Couldn't find file on head commit!"));
    }

    let mut walker = repo.revwalk()?;
    walker.push_head()?;
    walker.set_sorting(Sort::TIME)?;
    let mut current_commit = head_commit;

    for oid in walker {
        let commit = repo.find_commit(match oid {
            Ok(oid) => oid,
            Err(_) => return Err(Error::from_str("Couldn't find commit!")),
        })?;
        let tree_entry = match commit.tree()?.get_path(file_path) {
            Ok(entry) => entry,
            Err(_) => break,
        };

        // Find first object with same name but different SHA code.
        if tree_entry.id() != head_entry_id {
            break;
        }

        current_commit = commit;
    }

    Ok(current_commit.time().seconds())
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
    let root = Path::new("/");
    let mut current_path = path;
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
