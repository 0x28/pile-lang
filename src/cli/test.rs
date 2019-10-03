use super::read_program;

#[test]
fn test_cli() {
    assert_eq!(
        read_program(&["prog", "unknown.txt"]),
        Err("unknown.txt: No such file or directory (os error 2)".to_string())
    );

    assert_eq!(
        read_program(&["prog", "a", "b", "c"]),
        Err("Usage: prog [FILE]".to_string())
    );
}
