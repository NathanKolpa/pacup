use std::fs::File;
use std::path::PathBuf;

use pacup::paclist::{PackageLineKind, PackageListReader};

fn get_test_file_path(name: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("paclist");
    path.push(name);
    path
}

#[test]
fn test_read_empty_file() {
    let file = File::open(get_test_file_path("empty_list")).unwrap();
    let mut reader = PackageListReader::new(file);

    let result = reader.next_line();

    assert!(result.is_none());
}

#[test]
fn test_read_empty_file_with_comment() {
    let file = File::open(get_test_file_path("empty_list_with_comment")).unwrap();
    let mut reader = PackageListReader::new(file);

    let result = reader.next_line();

    assert!(result.is_none());
}

fn read_single_entry(file: &str) {
    let file = File::open(get_test_file_path(file)).unwrap();
    let mut reader = PackageListReader::new(file);

    let result = reader.next_line();
    let line = result.unwrap().unwrap();

    assert_eq!(line.name(), "xorg");
    assert_eq!(line.kind(), PackageLineKind::Package)
}

#[test]
fn test_read_single_entry() {
    read_single_entry("single_entry");
}

#[test]
fn test_read_single_entry_with_comment() {
    read_single_entry("single_entry_with_comment");
}

#[test]
fn test_read_single_entry_with_empty_lines() {
    read_single_entry("single_entry_with_empty_lines");
}

fn read_multiple_entries(file: &str) {
    let file = File::open(get_test_file_path(file)).unwrap();
    let mut reader = PackageListReader::new(file);

    let line = reader.next_line().unwrap().unwrap();
    assert_eq!(line.name(), "xorg");
    assert_eq!(line.kind(), PackageLineKind::Package);

    let line = reader.next_line().unwrap().unwrap();
    assert_eq!(line.name(), "zsh");
    assert_eq!(line.kind(), PackageLineKind::Aur);

    let line = reader.next_line().unwrap().unwrap();
    assert_eq!(line.name(), "grub");
    assert_eq!(line.kind(), PackageLineKind::Package);
}

#[test]
fn test_read_multiple_entries() {
    read_multiple_entries("multiple_entries");
}

#[test]
fn test_read_multiple_entries_with_comments() {
    read_multiple_entries("multiple_entries_with_comments");
}

#[test]
fn test_read_multiple_entries_with_empty_lines() {
    read_multiple_entries("multiple_entries_with_empty_lines");
}
