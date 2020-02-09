use super::*;

#[test]
fn test_read_program() {
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

// Depending on how the test is called the source can be different.
fn get_test_source() -> ProgramSource {
    if atty::is(Stream::Stdin) {
        ProgramSource::Repl
    } else {
        ProgramSource::Stdin
    }
}

#[test]
fn test_read_options1() {
    let options = read_options(vec!["test1"]);
    assert_eq!(options.stack_size(), 100);
    assert_eq!(options.trace(), false);
    assert_eq!(options.source().as_ref(), &get_test_source());
}

#[test]
fn test_read_options2() {
    let options = read_options(vec!["test2", "-"]);

    assert_eq!(options.stack_size(), 100);
    assert_eq!(options.trace(), false);
    assert_eq!(options.source().as_ref(), &ProgramSource::Stdin);
}

#[test]
fn test_read_options3() {
    let options = read_options(vec!["test3", "./some_program.pile"]);

    assert_eq!(options.stack_size(), 100);
    assert_eq!(options.trace(), false);
    assert_eq!(
        options.source().as_ref(),
        &ProgramSource::File(PathBuf::from("./some_program.pile"))
    );
}

#[test]
fn test_read_options4() {
    let options = read_options(vec!["test4", "-t", "--stack-size", "123"]);

    assert_eq!(options.stack_size(), 123);
    assert_eq!(options.trace(), true);
    assert_eq!(options.source().as_ref(), &get_test_source());
}
