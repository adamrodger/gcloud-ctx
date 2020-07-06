use assert_fs::prelude::*;
use common::TempConfigurationStore;
use predicates::prelude::*;

mod common;

#[test]
fn no_args_defaults_to_current() {
    let (mut cli, tmp) = TempConfigurationStore::new()
        .unwrap()
        .with_config("foo")
        .with_config_activated("bar")
        .build()
        .unwrap();

    cli.assert().success().stdout("bar\n");

    tmp.close().unwrap();
}

#[test]
fn unknown_subcommand_defaults_to_activate() {
    let (mut cli, tmp) = TempConfigurationStore::new()
        .unwrap()
        .with_config("foo")
        .with_config_activated("bar")
        .build()
        .unwrap();

    cli.arg("foo");

    cli.assert().success().stdout("Successfully activated 'foo'\n");
    tmp.child("active_config").assert("foo");

    tmp.close().unwrap();
}

#[test]
fn activate_known_configuration_succeeds() {
    let (mut cli, tmp) = TempConfigurationStore::new()
        .unwrap()
        .with_config("foo")
        .with_config_activated("bar")
        .build()
        .unwrap();

    cli.arg("activate").arg("foo");

    cli.assert().success().stdout("Successfully activated 'foo'\n");
    tmp.child("active_config").assert("foo");

    tmp.close().unwrap();
}

#[test]
fn activate_unknown_configuration_fails() {
    let (mut cli, tmp) = TempConfigurationStore::new()
        .unwrap()
        .with_config_activated("foo")
        .build()
        .unwrap();

    cli.arg("activate").arg("unknown");

    cli.assert()
        .failure()
        .stderr("Error: Unable to find configuration 'unknown'\n");
    tmp.child("active_config").assert("foo");

    tmp.close().unwrap();
}

#[test]
fn current_shows_active_configuration() {
    let (mut cli, tmp) = TempConfigurationStore::new()
        .unwrap()
        .with_config("foo")
        .with_config_activated("bar")
        .build()
        .unwrap();

    cli.arg("current");

    cli.assert().success().stdout("bar\n");

    tmp.close().unwrap();
}

#[test]
fn list_shows_configurations() {
    let (mut cli, tmp) = TempConfigurationStore::new()
        .unwrap()
        .with_config("foo")
        .with_config("bar")
        .with_config_activated("baz")
        .with_config("qux")
        .build()
        .unwrap();

    cli.arg("list");

    #[rustfmt::skip]
    let expected = vec![
        "  bar",
        "* baz",
        "  foo",
        "  qux",
        "",
    ].join("\n");

    cli.assert().success().stdout(expected);

    tmp.close().unwrap();
}

#[test]
fn rename_inactive_configuration_succeeds() {
    let (mut cli, tmp) = TempConfigurationStore::new()
        .unwrap()
        .with_config("foo")
        .with_config_activated("bar")
        .build()
        .unwrap();

    cli.arg("rename").arg("foo").arg("renamed");

    cli.assert()
        .success()
        .stdout("Successfully renamed configuration 'foo' to 'renamed'\n");

    tmp.child("active_config").assert("bar");
    tmp.child("configurations/config_foo")
        .assert(predicate::path::missing());
    tmp.child("configurations/config_renamed")
        .assert(predicate::path::exists());

    tmp.close().unwrap();
}

#[test]
fn rename_active_configuration_succeeds() {
    let (mut cli, tmp) = TempConfigurationStore::new()
        .unwrap()
        .with_config("foo")
        .with_config_activated("bar")
        .build()
        .unwrap();

    cli.arg("rename").arg("bar").arg("renamed");

    #[rustfmt::skip]
    cli.assert().success().stdout(vec![
        "Successfully renamed configuration 'bar' to 'renamed'",
        "Configuration 'renamed' is now active",
        "",
    ].join("\n"));

    tmp.child("active_config").assert("renamed");
    tmp.child("configurations/config_bar")
        .assert(predicate::path::missing());
    tmp.child("configurations/config_renamed")
        .assert(predicate::path::exists());

    tmp.close().unwrap();
}

#[test]
fn rename_to_existing_name_with_force_overwrites_existing() {
    let (mut cli, tmp) = TempConfigurationStore::new()
        .unwrap()
        .with_config("foo")
        .with_config_activated("bar")
        .build()
        .unwrap();

    cli.arg("rename").arg("bar").arg("foo").arg("--force");

    #[rustfmt::skip]
    cli.assert().success().stdout(vec![
        "Successfully renamed configuration 'bar' to 'foo'",
        "Configuration 'foo' is now active",
        "",
    ].join("\n"));

    tmp.child("active_config").assert("foo");
    tmp.child("configurations/config_bar")
        .assert(predicate::path::missing());
    tmp.child("configurations/config_foo").assert(predicate::path::exists());

    tmp.close().unwrap();
}

#[test]
fn rename_to_existing_name_without_force_fails() {
    let (mut cli, tmp) = TempConfigurationStore::new()
        .unwrap()
        .with_config("foo")
        .with_config_activated("bar")
        .build()
        .unwrap();

    cli.arg("rename").arg("bar").arg("foo");

    #[rustfmt::skip]
    cli.assert().failure().stderr("Error: A configuration named 'foo' already exists. Use --force to overwrite it\n");

    tmp.child("active_config").assert("bar");
    tmp.child("configurations/config_bar").assert(predicate::path::exists());
    tmp.child("configurations/config_foo").assert(predicate::path::exists());

    tmp.close().unwrap();
}

#[test]
fn rename_to_invalid_name_fails() {
    let (mut cli, tmp) = TempConfigurationStore::new()
        .unwrap()
        .with_config_activated("foo")
        .build()
        .unwrap();

    cli.arg("rename").arg("foo").arg("invalid_name");

    cli.assert()
        .failure()
        .stderr("Error: 'invalid_name' is invalid. Configuration names must only contain ASCII letters and numbers\n");

    tmp.child("active_config").assert("foo");
    tmp.child("configurations/config_foo").assert(predicate::path::exists());
    tmp.child("configurations/config_invalid_name")
        .assert(predicate::path::missing());

    tmp.close().unwrap();
}

#[test]
fn rename_unknown_configuration_fails() {
    let (mut cli, tmp) = TempConfigurationStore::new()
        .unwrap()
        .with_config_activated("foo")
        .build()
        .unwrap();

    cli.arg("rename").arg("unknown").arg("bar");

    cli.assert()
        .failure()
        .stderr("Error: Unable to find configuration 'unknown'\n");

    tmp.child("active_config").assert("foo");
    tmp.child("configurations/config_foo").assert(predicate::path::exists());
    tmp.child("configurations/config_bar")
        .assert(predicate::path::missing());

    tmp.close().unwrap();
}

#[test]
fn create_sets_properties_successfully() {
    let (mut cli, tmp) = TempConfigurationStore::new()
        .unwrap()
        .with_config_activated("foo")
        .build()
        .unwrap();

    #[rustfmt::skip]
    cli.arg("create")
       .arg("new-config")
       .args(&["--project", "my-project"])
       .args(&["--account", "a.user@example.org"])
       .args(&["--zone", "europe-west1-d"])
       .args(&["--region", "us-east1"]);

    cli.assert()
        .success()
        .stdout("Successfully created configuration 'new-config'\n");

    #[rustfmt::skip]
    tmp.child("configurations/config_new-config").assert([
        "[core]",
        "project=my-project",
        "account=a.user@example.org",
        "[compute]",
        "zone=europe-west1-d",
        "region=us-east1",
        ""
    ].join("\n"));

    tmp.close().unwrap();
}

#[test]
fn create_without_activate_maintains_previous_activation() {
    let (mut cli, tmp) = TempConfigurationStore::new()
        .unwrap()
        .with_config_activated("foo")
        .build()
        .unwrap();

    #[rustfmt::skip]
    cli.arg("create")
       .arg("new-config")
       .args(&["--project", "my-project"])
       .args(&["--account", "a.user@example.org"])
       .args(&["--zone", "europe-west1-d"])
       .args(&["--region", "us-east1"]);

    cli.assert()
        .success()
        .stdout("Successfully created configuration 'new-config'\n");

    tmp.child("active_config").assert("foo");

    tmp.close().unwrap();
}

#[test]
fn create_with_activate_activates_new_configuration() {
    let (mut cli, tmp) = TempConfigurationStore::new()
        .unwrap()
        .with_config_activated("foo")
        .build()
        .unwrap();

    #[rustfmt::skip]
    cli.arg("create")
       .arg("new-config")
       .args(&["--project", "my-project"])
       .args(&["--account", "a.user@example.org"])
       .args(&["--zone", "europe-west1-d"])
       .args(&["--region", "us-east1"])
       .arg("--activate");

    cli.assert().success().stdout(
        "Successfully created configuration 'new-config'\n\
         Configuration 'new-config' is now active\n",
    );

    tmp.child("active_config").assert("new-config");

    tmp.close().unwrap();
}

#[test]
fn create_with_force_succeeds() {
    let (mut cli, tmp) = TempConfigurationStore::new()
        .unwrap()
        .with_config_activated("foo")
        .build()
        .unwrap();

    #[rustfmt::skip]
    cli.arg("create")
       .arg("foo")
       .args(&["--project", "my-project"])
       .args(&["--account", "a.user@example.org"])
       .args(&["--zone", "europe-west1-d"])
       .args(&["--region", "us-east1"])
       .arg("--force");

    cli.assert()
        .success()
        .stdout("Successfully created configuration 'foo'\n");

    tmp.child("active_config").assert("foo");

    #[rustfmt::skip]
    tmp.child("configurations/config_foo").assert([
        "[core]",
        "project=my-project",
        "account=a.user@example.org",
        "[compute]",
        "zone=europe-west1-d",
        "region=us-east1",
        ""
    ].join("\n"));

    tmp.close().unwrap();
}

#[test]
fn create_with_invalid_name_fails() {
    let (mut cli, tmp) = TempConfigurationStore::new()
        .unwrap()
        .with_config_activated("foo")
        .build()
        .unwrap();

    #[rustfmt::skip]
    cli.arg("create")
       .arg("invalid_name")
       .args(&["--project", "my-project"])
       .args(&["--account", "a.user@example.org"])
       .args(&["--zone", "europe-west1-d"])
       .args(&["--region", "us-east1"]);

    cli.assert()
        .failure()
        .stderr("Error: 'invalid_name' is invalid. Configuration names must only contain ASCII letters and numbers\n");

    tmp.close().unwrap();
}

#[test]
fn create_without_force_fails() {
    let (mut cli, tmp) = TempConfigurationStore::new()
        .unwrap()
        .with_config_activated("foo")
        .build()
        .unwrap();

    #[rustfmt::skip]
    cli.arg("create")
       .arg("foo")
       .args(&["--project", "my-project"])
       .args(&["--account", "a.user@example.org"])
       .args(&["--zone", "europe-west1-d"])
       .args(&["--region", "us-east1"]);

    cli.assert()
        .failure()
        .stderr("Error: A configuration named 'foo' already exists. Use --force to overwrite it\n");

    tmp.close().unwrap();
}
