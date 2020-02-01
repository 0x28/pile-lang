use super::*;

#[test]
fn test_cli() {
    let options = CommandLineOptions {
        stack_size: 100,
        source: Rc::new(ProgramSource::File(PathBuf::from("unknown.txt"))),
        trace: true,
    };

    assert_eq!(
        options.read_program(),
        Err("unknown.txt: No such file or directory (os error 2)".to_string())
    );
}
