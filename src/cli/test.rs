use super::CommandLineOptions;

#[test]
fn test_cli() {
    let options = CommandLineOptions {
        stack_size: 100,
        program: Some("unknown.txt".to_owned()),
        debug: true,
    };

    assert_eq!(
        options.read_program(),
        Err("unknown.txt: No such file or directory (os error 2)".to_string())
    );
}
