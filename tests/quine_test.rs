use std::fs;
use std::process::Command;

#[test]
fn test_quine() {
    let quine_file =
        env!("CARGO_MANIFEST_DIR").to_owned() + "/src/examples/quine.pile";

    let pile_interpreter = env!("CARGO_BIN_EXE_pile");

    let output = Command::new(pile_interpreter)
        .args(&[&quine_file])
        .output()
        .expect("Couldn't run quine test!");

    let output = String::from_utf8_lossy(&output.stdout);

    assert_eq!(output, fs::read_to_string(quine_file).unwrap());
}
