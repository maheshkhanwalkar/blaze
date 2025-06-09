use crate::file::save_file;
use crate::vfs::BLAZE_REPOSITORY_DIR;
use serde::{Deserialize, Serialize};
use sha2::digest::Update;
use sha2::{Digest, Sha512};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::UNIX_EPOCH;

type CommitHash = String;

#[derive(Serialize, Deserialize)]
pub struct Reference {
    path_map: HashMap<String, CommitHash>,
}

#[derive(Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct Commit {
    pub author: Author,
    pub title: String,
    pub message: String,
    pub timestamp: u64,
    pub reference: Reference,
}

impl Commit {
    fn new(author: Author, title: String, message: String, reference: Reference) -> Self {
        Commit {
            author,
            title,
            message,
            reference,
            timestamp: UNIX_EPOCH.elapsed().unwrap().as_secs(),
        }
    }

    pub fn load(hash: &CommitHash) -> Self {
        let commit_str = read_to_string(commit_path(hash)).expect("failed to load commit");
        let commit: Commit = serde_json::from_str(&commit_str).unwrap();
        commit
    }

    pub fn save(&mut self) {
        let hash = compute_hash(self);
        let commit_str = serde_json::to_string(self).unwrap();

        save_file(commit_path(&hash).as_str(), &commit_str)
    }
}

fn commit_path(hash: &CommitHash) -> String {
    format!("{BLAZE_REPOSITORY_DIR}/commits/{}", hash)
}

fn compute_hash(commit: &Commit) -> CommitHash {
    let mut hasher = Sha512::new();

    Update::update(&mut hasher, commit.author.name.as_bytes());
    Update::update(&mut hasher, commit.author.email.as_bytes());
    Update::update(&mut hasher, commit.title.as_bytes());
    Update::update(&mut hasher, commit.message.as_bytes());
    Update::update(&mut hasher, commit.timestamp.to_le_bytes().as_slice());

    let hash = hasher.finalize();
    let hash_str = format!("{:x}", hash);

    hash_str
}
