use std::path::{Path, PathBuf};

use eir_core::error::Result;

pub struct Merger {
    left: PathBuf,
    right: PathBuf,
}

impl Merger {
    pub fn new(left: impl AsRef<Path>, right: impl AsRef<Path>) -> Self {
        Self {
            left: left.as_ref().to_path_buf(),
            right: right.as_ref().to_path_buf(),
        }
    }

    pub fn merge(&self, output: impl AsRef<Path>) -> Result<MergeReport> {
        let _output = output.as_ref();

        // Implementation comes next.
        todo!("merge .eir + .eir")
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct MergeReport {
    pub entities_added: usize,
    pub entities_combined: usize,
    pub entities_skipped: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_does_not_modify_inputs() {
        // Create A.eir and B.eir.
        // Record their sizes/contents.
        // Merge into C.eir.
        // Verify A and B are unchanged.
    }

    #[test]
    fn merge_creates_new_database() {
        // A + B -> C
        // Verify C exists and can be opened by eir-core.
    }

    #[test]
    fn merge_rejects_same_output_as_input() {
        // A + B -> A must fail.
    }

    #[test]
    fn merge_is_atomic_on_failure() {
        // Failed merge must not leave a partially-written C.eir.
    }
}
