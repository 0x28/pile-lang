use super::*;

#[test]
fn test_read_program() {
    let options = CommandLineOptions {
        stack_size: 100,
        source: Rc::new(ProgramSource::File(PathBuf::from("unknown.txt"))),
        trace: true,
        format: false,
        completion: None,
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
fn test_read_options1() -> Result<(), String> {
    let options = read_options(vec!["test1"])?;
    assert_eq!(options.stack_size(), 100);
    assert_eq!(options.trace(), false);
    assert_eq!(options.format(), false);
    assert_eq!(options.source().as_ref(), &get_test_source());
    assert_eq!(options.completion, None);

    Ok(())
}

#[test]
fn test_read_options2() -> Result<(), String> {
    let options = read_options(vec!["test2", "-"])?;

    assert_eq!(options.stack_size(), 100);
    assert_eq!(options.trace(), false);
    assert_eq!(options.format(), false);
    assert_eq!(options.source().as_ref(), &ProgramSource::Stdin);

    Ok(())
}

#[test]
fn test_read_options3() -> Result<(), String> {
    let options = read_options(vec!["test3", "./some_program.pile"])?;

    assert_eq!(options.stack_size(), 100);
    assert_eq!(options.trace(), false);
    assert_eq!(options.format(), false);
    assert_eq!(
        options.source().as_ref(),
        &ProgramSource::File(PathBuf::from("./some_program.pile"))
    );

    Ok(())
}

#[test]
fn test_read_options4() -> Result<(), String> {
    let options = read_options(vec!["test4", "-t", "--stack-size", "123"])?;

    assert_eq!(options.stack_size(), 123);
    assert_eq!(options.trace(), true);
    assert_eq!(options.format(), false);
    assert_eq!(options.source().as_ref(), &get_test_source());

    Ok(())
}

#[test]
fn test_read_options5() {
    let options = read_options(vec!["test5", "-t", "--stack-size", "yes"]);

    assert!(options.is_err());
    assert!(options
        .unwrap_err()
        .contains("The value must be a natural number"));
}

#[test]
fn test_read_completion1() -> Result<(), String> {
    let completion =
        read_options(vec!["test6", "-c", "prefix_", "100", "file.pile"])?
            .completion
            .unwrap();

    assert_eq!(completion.prefix, "prefix_");
    assert_eq!(completion.line, 100);

    Ok(())
}

#[test]
fn test_read_completion2() {
    let options =
        read_options(vec!["test7", "--complete", "var_", "nan", "file.pile"]);

    assert!(options.is_err());
    assert!(options
        .unwrap_err()
        .contains("error parsing <line> in '--complete':"));
}

#[test]
fn test_read_format() {
    let options = read_options(vec!["test8", "--format", "-"]);
    assert_eq!(options.unwrap().format(), true);

    let options = read_options(vec!["test8", "-f", "test.pile"]);
    assert_eq!(options.unwrap().format(), true);

    let options = read_options(vec!["test8", "-f"]);
    assert!(options.is_err());
}
