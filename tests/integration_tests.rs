use std::process::Command;

#[test]
fn test_add_todo_item() {
    let output = Command::new("cargo")
        .args(&["run", "--", "add", "Test todo item"])
        .output()
        .expect("Failed to execute command");

    assert!(String::from_utf8_lossy(&output.stdout).contains("Added new todo item"));
}

#[test]
fn test_list_todo_items() {
    let output = Command::new("cargo")
        .args(&["run", "--", "list"])
        .output()
        .expect("Failed to execute command");

    assert!(String::from_utf8_lossy(&output.stdout).contains("Test todo item"));
}