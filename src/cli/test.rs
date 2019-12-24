use super::*;

#[test]
fn test_cli() {
    let options = CommandLineOptions {
        stack_size: 100,
        source: ProgramSource::File(PathBuf::from("unknown.txt")),
        debug: true,
    };

    assert_eq!(
        options.read_program(),
        Err("unknown.txt: No such file or directory (os error 2)".to_string())
    );
}
