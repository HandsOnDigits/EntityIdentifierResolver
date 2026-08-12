use std::path::Path;

use crate::{Error, Result};

pub(crate) fn validate_merge_paths(left: &Path, right: &Path, output: &Path) -> Result<()> {
    if left == right {
        return Err(Error::InvalidInput(
            "left and right databases must be different".into(),
        ));
    }

    if output == left || output == right {
        return Err(Error::OutputInputCollision);
    }

    if !left.exists() {
        return Err(Error::LeftDatabaseMissing(left.display().to_string()));
    }

    if !right.exists() {
        return Err(Error::RightDatabaseMissing(right.display().to_string()));
    }

    if output.exists() {
        return Err(Error::OutputExists(output.display().to_string()));
    }

    Ok(())
}
