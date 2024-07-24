use std::fs;
use std::io::prelude::*;
use std::process::Command;   // Run programs
use assert_cmd::prelude::*;  // Add methods on commands
use assert_fs::fixture::*;   // TempDir, NamedTempFile
use predicates::prelude::*;  // Used for writing assertions
use serial_test::serial;

use clipboard_win::{formats, set_clipboard, get_clipboard};

fn set_clipboard_and_verify(text_clipboard : &str) {
    set_clipboard(formats::Unicode, text_clipboard).expect("To set clipboard");
    // verify
    let contents: String = get_clipboard(formats::Unicode).expect("To get clipboard");
    assert_eq!(text_clipboard, contents);
}

// Neither APPEND nor OUTPUT
#[test]
#[allow(dead_code)]
#[serial]
fn read_clipboard() -> Result<(), Box<dyn std::error::Error>> {
    let text_clipboard = "read_clipboard():\nActual content\nfor test\n";
    set_clipboard_and_verify(&text_clipboard);

    let mut cmd= Command::cargo_bin("winclip")?;
    cmd.arg("--paste")
        .assert()
        .success()
        .stdout(text_clipboard.to_string())
        .stderr("");

    Ok(())
}

// OUTPUT only, file does not exist, it is created.
#[test]
#[allow(dead_code)]
#[serial]
fn output_to_file() -> Result<(), Box<dyn std::error::Error>> {
    let fname = "output_to_file";
    let text_clipboard = format!("{fname}():\nActual content\nfor test\n");
    set_clipboard_and_verify(&text_clipboard);

    let tmp_dir = TempDir::new().unwrap();
    let target_path = tmp_dir.path().join(format!("{fname}.txt"));

    let mut cmd = Command::cargo_bin("winclip")?;
    let target_file = target_path.to_str().expect("Get String from PathBuf");
    // cmd.arg("--paste").arg(target_file);
    cmd.args(["--paste", &target_file]);
    cmd.assert()
        .success()
        .stdout("")
        .stderr("");

    let contents = fs::read_to_string(&target_file)
        .expect("Unable to read contents of file '{&target_file}'");
    assert_eq!(text_clipboard, contents);

    tmp_dir.close().unwrap();
    Ok(())
}
/*
// APPEND only, destination directory does not exist => error
#[test]
#[allow(dead_code)]
#[serial]
fn append_destination_directory_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let text_clipboard = "append_destination_directory_doesnt_exist():\nActual content\nfor test\n";
    set_clipboard_and_verify(&text_clipboard);

    let mut cmd = Command::cargo_bin("winclip")?;
    cmd.arg("--append").arg("destination/directory/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Unable to open & append file"));

    Ok(())
}

// APPEND only, file exist with content, it is created.
#[test]
#[allow(dead_code)]
#[serial]
fn append_file_doesnt_exist_created() -> Result<(), Box<dyn std::error::Error>> {
    let fname = "append_file_doesnt_exist_created";
    let text_clipboard = format!("{fname}():\nActual content\nfor test\n");
    set_clipboard_and_verify(&text_clipboard);

    let tmp_dir = TempDir::new().unwrap();
    let target_file = tmp_dir.path().join(format!("{fname}.txt"));

    let mut cmd = Command::cargo_bin("winclip")?;
    cmd.arg("--append").arg(&target_file);
    cmd.assert()
        .success()
        .stdout("")
        .stderr("");

    let contents = fs::read_to_string(target_file).expect("Unable to read file");
    assert_eq!(text_clipboard, contents);

    tmp_dir.close().unwrap();
    Ok(())
}

// APPEND only, file exists, it is appended to.
#[test]
#[allow(dead_code)]
#[serial]
fn append_file_exists() -> Result<(), Box<dyn std::error::Error>> {
    let fname = "append_file_exists";
    let tmp_dir = TempDir::new().unwrap();
    let target_file = tmp_dir.path().join(format!("{fname}.txt"));

    let initial_content = format!("{fname}()\n1: Initial content\n2: second line\n");
    let text_clipboard = format!("{fname}():\n3: Actual content\n4: for test\n");
    let text_expected = initial_content.to_string() + &text_clipboard;

    let mut file = fs::File::create(&target_file)?;
    file.write_all(initial_content.as_ref())?;

    let result = append_only(&text_clipboard, &text_expected,
                             &target_file.display().to_string());

    tmp_dir.close().unwrap();

    result
}

// APPEND only, file exists, it is appended to.
#[test]
#[allow(dead_code)]
#[serial]
fn append_file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let fname = "append_file_doesnt_exist";
    let tmp_dir = TempDir::new().unwrap();
    let target_file = tmp_dir.path().join(format!("{fname}.txt"));

    let text_clipboard = format!("{fname}():\n1: Actual content\n2: for test\n");

    let result = append_only(&text_clipboard, &text_clipboard,
                             &target_file.display().to_string());

    tmp_dir.close().unwrap();

    result
}

fn append_only(text_clipboard : &str, text_expected : &str, target_file : &str) -> Result<(), Box<dyn std::error::Error>> {
    set_clipboard_and_verify(text_clipboard);

    let mut cmd = Command::cargo_bin("winclip")?;
    cmd.arg("--append").arg(target_file);
    cmd.assert()
        .success()
        .stdout("")
        .stderr("");

    let contents = fs::read_to_string(target_file).expect("Unable to read file");
    assert_eq!(text_expected, contents);

    Ok(())
}

// APPEND and OUTPUT
#[test]
#[allow(dead_code)]
#[serial]
fn append_and_output() -> Result<(), Box<dyn std::error::Error>> {
    let fname = "append_and_output";

    let tmp_dir = TempDir::new().unwrap();
    let append_file = tmp_dir.path().join(format!("{fname}.append.txt"));
    let create_file = tmp_dir.path().join(format!("{fname}.create.txt"));

    let initial_content = format!("{fname}()\n1: Initial content\n2: second line\n");
    let text_clipboard = format!("{fname}():\n3: Actual content\n4: for test\n");
    let text_append = initial_content.to_string() + &text_clipboard;

    let mut file = fs::File::create(&append_file)?;
    file.write_all(initial_content.as_ref())?;

    set_clipboard_and_verify(&text_clipboard);

    let mut cmd = Command::cargo_bin("winclip")?;
    cmd.arg("--append").arg(&append_file);
    cmd.arg("--out").arg(&create_file);
    cmd.assert()
        .success()
        .stdout("")
        .stderr("");

    let contents = fs::read_to_string(&append_file).expect("Unable to read file");
    assert_eq!(text_append, contents);

    let contents = fs::read_to_string(&create_file).expect("Unable to read file");
    assert_eq!(text_clipboard, contents);

    tmp_dir.close().unwrap();

    Ok(())
}

*/
