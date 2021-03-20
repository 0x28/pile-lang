use std::io::Write;
use std::process::Command;
use std::process::Stdio;

#[test]
fn test_readlines() {
    let readlines_file =
        env!("CARGO_MANIFEST_DIR").to_owned() + "/tests/upcase.pile";

    let pile_interpreter = env!("CARGO_BIN_EXE_pile");

    let mut child = Command::new(pile_interpreter)
        .args(&[&readlines_file])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Couldn't run readline test!");

    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin
            .write_all("Hi and bye!\nquit\n".as_bytes())
            .expect("Failed to write to stdin");
    }

    let output = child.wait_with_output().expect("Failed to write to stdin");

    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "=> HI AND BYE!\n=> QUIT\n"
    );
}
