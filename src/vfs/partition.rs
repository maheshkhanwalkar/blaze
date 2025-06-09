use crate::file::save_file;
use crate::vfs::commit::CommitHash;
use crate::vfs::BLAZE_REPOSITORY_DIR;
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;

struct Partition {
    path: String,
    latest_commit: CommitHash,
}

#[derive(Serialize, Deserialize)]
struct CommitGraph {
    parents: Vec<CommitHash>,
    children: Vec<CommitHash>,
}

impl Partition {
    /// Links a commit to its parent in the commit graph.
    ///
    /// This function updates the commit graph by linking the given `commit`
    /// to its `parent`. Specifically, it updates the parent commit's graph to
    /// include the `commit` as a child, and creates a new graph entry for
    /// the `commit` with the `parent` specified.
    pub fn link_commit(&mut self, commit: CommitHash, parent: CommitHash) {
        let parent_path = self.commit_graph_path(&parent);
        let commit_path = self.commit_graph_path(&commit);

        // Update the parent commit's graph
        let mut parent_graph = read_commit_graph(&parent_path);
        parent_graph.children.push(commit);
        write_commit_graph(&parent_path, &parent_graph);

        // Create the new commit's graph
        let commit_graph = CommitGraph {
            parents: vec![parent],
            children: vec![],
        };
        write_commit_graph(&commit_path, &commit_graph);
    }

    fn commit_graph_path(&self, hash: &CommitHash) -> String {
        format!(
            "{BLAZE_REPOSITORY_DIR}/partition/{}/graph/{}",
            self.path, hash
        )
    }
}

fn read_commit_graph(path: &String) -> CommitGraph {
    let content = read_to_string(path).expect("could not read commit graph");
    serde_json::from_str(&content).unwrap()
}

fn write_commit_graph(path: &String, edge: &CommitGraph) {
    save_file(path.as_str(), &serde_json::to_string(&edge).unwrap());
}
