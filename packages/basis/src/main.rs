use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
use git2::{Commit, ObjectType, Repository, Sort};
use std::path::Path;

// https://github.com/rust-lang/mdBook/pull/1506
fn main() -> std::io::Result<()> {
    let files = [
        "packages/basis/src/main.rs",
        ".github/workflows/ci.yml",
        "packages/basis/Cargo.toml",
        "book.toml",
        "Cargo.toml",
        "docs/SUMMARY.md",
        "docs/README.md",
        "Cargo.lock",
        "SUMMARY.md",
        "README.md",
        "NOT_FOUND.md",
    ];

    for file in files {
        find_git_timestamp(file);
    }

    Ok(())
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
    let repo = match Repository::open("./.git") {
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
        if tree_entry.id().ne(&head_tree_entry_id) {
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
