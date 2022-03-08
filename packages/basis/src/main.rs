use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
use git2::{Commit, ObjectType, Oid, Repository, Sort};
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
        get_git_timestamp(file);
    }

    Ok(())
}

fn get_git_timestamp(file_name: &str) {
    let file_path = Path::new(file_name);
    let repo = Repository::open("./.git").expect("Couldn't open repository!");
    let head_commit = get_head_commit(&repo).expect("Couldn't find head commit!");
    let mut current_commit = head_commit;
    let mut head_tree_entry_id = Oid::zero();
    let mut head_walker = repo.revwalk().expect("Could't setup revwalk!");
    head_walker
        .push_head()
        .expect("Couldn't push first commit!");
    head_walker
        .set_sorting(Sort::TIME)
        .expect("Couldn't walk by time!");

    for commit_oid in head_walker {
        let oid = commit_oid.expect("Failed to revwalk!");
        let commit = repo.find_commit(oid).expect("Couldn't find commit!");
        let tree = commit.tree().expect("Couldn't find tree!");
        let tree_entry = match tree.get_path(file_path) {
            Ok(entry) => entry,
            Err(_) => continue,
        };

        if tree_entry.id().is_zero() == false {
            head_tree_entry_id = tree_entry.id();
            current_commit = commit;
            break;
        }
    }

    if head_tree_entry_id.is_zero() {
        println!("Couldn't find {} on head commit!", file_name);
        return;
    }

    let mut walker = repo.revwalk().expect("Could't setup revwalk!");
    walker.push_head().expect("Couldn't push first commit!");
    walker
        .set_sorting(Sort::TIME)
        .expect("Couldn't walk by time!");

    for commit_oid in walker {
        let oid = commit_oid.expect("Failed to revwalk!");
        let commit = repo.find_commit(oid).expect("Couldn't find commit!");
        let tree = commit.tree().expect("Couldn't find tree!");
        let tree_entry = match tree.get_path(file_path) {
            Ok(entry) => entry,
            Err(_) => continue,
        };

        if tree_entry.id().ne(&head_tree_entry_id) {
            break;
        }

        current_commit = commit;
    }

    display_commit(file_name, &current_commit);
}

fn get_head_commit(repo: &Repository) -> Result<Commit, git2::Error> {
    let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    obj.into_commit()
        .map_err(|_| git2::Error::from_str("Couldn't find commit!"))
}

fn display_commit(file_name: &str, commit: &Commit) {
    let china_timezone = FixedOffset::east(8 * 3600);
    let timestamp = commit.time().seconds();
    let utc_time = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(timestamp, 0), Utc);
    let china_time = utc_time.with_timezone(&china_timezone);
    println!(
        "Commit {} at: {}.",
        file_name,
        china_time.format("%Y-%m-%d %H:%M:%S"),
    );
}
