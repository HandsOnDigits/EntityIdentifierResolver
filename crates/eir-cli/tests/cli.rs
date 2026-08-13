use assert_cmd::Command;
use predicates::prelude::*;

fn eir() -> Command {
    Command::cargo_bin("eir").unwrap()
}

#[test]
fn help_succeeds() {
    eir()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Entity Identifier Resolver"))
        .stdout(predicate::str::contains("init"))
        .stdout(predicate::str::contains("build"))
        .stdout(predicate::str::contains("stats"))
        .stdout(predicate::str::contains("inspect"))
        .stdout(predicate::str::contains("search"))
        .stdout(predicate::str::contains("insert"))
        .stdout(predicate::str::contains("remove"))
        .stdout(predicate::str::contains("merge"))
        .stdout(predicate::str::contains("server"))
        .stdout(predicate::str::contains("completions"));
}

#[test]
fn database_lifecycle() {
    let temp = tempfile::tempdir().unwrap();

    let parent = temp.path();
    let database = parent.join("nutrition");
    let database_file = database.join("nutrition.eir");

    let fixture = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/test-entity.json");

    eir()
        .args(["init"])
        .arg(parent)
        .arg("nutrition")
        .assert()
        .success();

    assert!(database_file.exists());
    assert!(database.join("eir.toml").exists());
    assert!(database.join("segments").exists());
    assert!(database.join("wal").exists());

    eir()
        .args(["insert"])
        .arg(&database_file)
        .arg(&fixture)
        .assert()
        .success();

    eir()
        .args(["inspect"])
        .arg(&database_file)
        .args(["9100"])
        .assert()
        .success()
        .stdout(predicate::str::contains("9100"))
        .stdout(predicate::str::contains("Test Berry"));

    eir()
        .args(["search"])
        .arg(&database_file)
        .arg("Test Berry")
        .assert()
        .success()
        .stdout(predicate::str::contains("Test Berry"))
        .stdout(predicate::str::contains("score=1.00"));

    eir()
        .args(["remove"])
        .arg(&database_file)
        .args(["9100"])
        .assert()
        .success();

    eir()
        .args(["inspect"])
        .arg(&database_file)
        .args(["9100"])
        .assert()
        .success()
        .stdout(predicate::str::contains("not found"));

    eir()
        .args(["search"])
        .arg(&database_file)
        .arg("Test Berry")
        .assert()
        .success()
        .stdout(predicate::str::contains("Search: Test Berry"))
        .stdout(predicate::str::contains("score=").not());
}

#[test]
fn merge_combines_databases() {
    let temp = tempfile::tempdir().unwrap();

    let parent = temp.path();

    let left = parent.join("left");
    let right = parent.join("right");
    let output = parent.join("merged");

    let left_file = left.join("left.eir");
    let right_file = right.join("right.eir");
    let output_file = output.join("merged.eir");

    let fixture = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/test-entity.json");

    eir()
        .args(["init"])
        .arg(parent)
        .arg("left")
        .assert()
        .success();

    eir()
        .args(["init"])
        .arg(parent)
        .arg("right")
        .assert()
        .success();

    eir()
        .args(["insert"])
        .arg(&left_file)
        .arg(&fixture)
        .assert()
        .success();

    eir()
        .args(["insert"])
        .arg(&right_file)
        .arg(&fixture)
        .assert()
        .success();

    eir()
        .args(["merge"])
        .arg(&left_file)
        .arg(&right_file)
        .arg(&output_file)
        .assert()
        .success()
        .stdout(predicate::str::contains("Merge complete."))
        .stdout(predicate::str::contains("Entities added:"))
        .stdout(predicate::str::contains("Entities skipped:"));

    assert!(output_file.exists());

    eir()
        .args(["inspect"])
        .arg(&output_file)
        .arg("9100")
        .assert()
        .success()
        .stdout(predicate::str::contains("9100"))
        .stdout(predicate::str::contains("Test Berry"));
}

#[test]
fn merge_skips_duplicate_entity_ids() {
    let temp = tempfile::tempdir().unwrap();

    let parent = temp.path();

    let left = parent.join("left");
    let right = parent.join("right");
    let output = parent.join("merged");

    let left_file = left.join("left.eir");
    let right_file = right.join("right.eir");
    let output_file = output.join("merged.eir");

    let fixture = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/test-entity.json");

    eir()
        .args(["init"])
        .arg(parent)
        .arg("left")
        .assert()
        .success();

    eir()
        .args(["init"])
        .arg(parent)
        .arg("right")
        .assert()
        .success();

    eir()
        .args(["insert"])
        .arg(&left_file)
        .arg(&fixture)
        .assert()
        .success();

    eir()
        .args(["insert"])
        .arg(&right_file)
        .arg(&fixture)
        .assert()
        .success();

    eir()
        .args(["merge"])
        .arg(&left_file)
        .arg(&right_file)
        .arg(&output_file)
        .assert()
        .success()
        .stdout(predicate::str::contains("Entities added: 1"))
        .stdout(predicate::str::contains("Entities skipped: 1"));
}

#[test]
fn merge_rejects_existing_output() {
    let temp = tempfile::tempdir().unwrap();

    let parent = temp.path();

    let left = parent.join("left");
    let right = parent.join("right");

    let left_file = left.join("left.eir");
    let right_file = right.join("right.eir");

    let fixture = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/test-entity.json");

    eir()
        .args(["init"])
        .arg(parent)
        .arg("left")
        .assert()
        .success();

    eir()
        .args(["init"])
        .arg(parent)
        .arg("right")
        .assert()
        .success();

    eir()
        .args(["insert"])
        .arg(&left_file)
        .arg(&fixture)
        .assert()
        .success();

    eir()
        .args(["insert"])
        .arg(&right_file)
        .arg(&fixture)
        .assert()
        .success();

    let output = parent.join("output");

    eir()
        .args(["init"])
        .arg(parent)
        .arg("output")
        .assert()
        .success();

    let output_file = output.join("output.eir");

    eir()
        .args(["merge"])
        .arg(&left_file)
        .arg(&right_file)
        .arg(&output_file)
        .assert()
        .failure()
        .stderr(predicate::str::contains("already exists"));
}

#[test]
fn merge_rejects_output_input_collision() {
    let temp = tempfile::tempdir().unwrap();

    let parent = temp.path();

    let left = parent.join("left");
    let right = parent.join("right");

    let left_file = left.join("left.eir");
    let right_file = right.join("right.eir");

    let fixture = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/test-entity.json");

    eir()
        .args(["init"])
        .arg(parent)
        .arg("left")
        .assert()
        .success();

    eir()
        .args(["init"])
        .arg(parent)
        .arg("right")
        .assert()
        .success();

    eir()
        .args(["insert"])
        .arg(&left_file)
        .arg(&fixture)
        .assert()
        .success();

    eir()
        .args(["insert"])
        .arg(&right_file)
        .arg(&fixture)
        .assert()
        .success();

    eir()
        .args(["merge"])
        .arg(&left_file)
        .arg(&right_file)
        .arg(&left_file)
        .assert()
        .failure()
        .stderr(predicate::str::contains("output"));
}

#[test]
fn update_replaces_entity_and_persists() {
    let temp = tempfile::tempdir().unwrap();

    let parent = temp.path();
    let database = parent.join("nutrition");
    let database_file = database.join("nutrition.eir");

    let fixture = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/test-entity.json");

    let update_fixture = parent.join("update.json");

    std::fs::write(
        &update_fixture,
        r#"
[
  {
    "id": 9100,
    "aliases": ["Updated Berry"],
    "tags": [],
    "attributes": [],
    "relationships": [],
    "sources": []
  }
]
"#,
    )
    .unwrap();

    // Create database.
    eir()
        .args(["init"])
        .arg(parent)
        .arg("nutrition")
        .assert()
        .success();

    // Insert the original entity.
    eir()
        .args(["insert"])
        .arg(&database_file)
        .arg(&fixture)
        .assert()
        .success();

    // Verify the original identity is searchable.
    eir()
        .args(["search"])
        .arg(&database_file)
        .arg("Test Berry")
        .assert()
        .success()
        .stdout(predicate::str::contains("Test Berry"))
        .stdout(predicate::str::contains("score=1.00"));

    // Replace the entity.
    eir()
        .args(["update"])
        .arg(&database_file)
        .arg("9100")
        .args(["--input"])
        .arg(&update_fixture)
        .assert()
        .success()
        .stdout(predicate::str::contains("Updated entity 9100"));

    // The new identity must be searchable.
    eir()
        .args(["search"])
        .arg(&database_file)
        .arg("Updated Berry")
        .assert()
        .success()
        .stdout(predicate::str::contains("Updated Berry"))
        .stdout(predicate::str::contains("score=1.00"));

    // The old identity must no longer be searchable.
    eir()
        .args(["search"])
        .arg(&database_file)
        .arg("Test Berry")
        .assert()
        .success()
        .stdout(predicate::str::contains("Search: Test Berry"))
        .stdout(predicate::str::contains("score=").not());

    // Verify the stored entity.
    eir()
        .args(["inspect"])
        .arg(&database_file)
        .arg("9100")
        .assert()
        .success()
        .stdout(predicate::str::contains("9100"))
        .stdout(predicate::str::contains("Updated Berry"))
        .stdout(predicate::str::contains("Test Berry").not());

    // Reopen through a new CLI process and verify persistence.
    eir()
        .args(["search"])
        .arg(&database_file)
        .arg("Updated Berry")
        .assert()
        .success()
        .stdout(predicate::str::contains("Updated Berry"))
        .stdout(predicate::str::contains("score=1.00"));
}
