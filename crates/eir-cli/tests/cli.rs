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
        .args(["--entity", "9100"])
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
        .args(["--entity", "9100"])
        .assert()
        .success();

    eir()
        .args(["inspect"])
        .arg(&database_file)
        .args(["--entity", "9100"])
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
