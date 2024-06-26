use super::*;
use crate::lex::Lexer;
use crate::program_source::ProgramSource;

use std::fs;
use std::path::PathBuf;
use std::rc::Rc;

fn expect_formatting(original: &str, formatted: &str) {
    let lexer = Lexer::new(original, Rc::new(ProgramSource::Stdin));
    let mut result = Vec::<u8>::new();

    write_formatting(&mut result, lexer).unwrap();

    let result = String::from_utf8(result).unwrap();
    if result != formatted {
        eprintln!("===\n{}\n===\n{}\n===", result, formatted);
        assert_eq!(result, formatted);
    }
}

#[test]
fn test_format_comment1() {
    expect_formatting(
        "# never change comments!\n2 + print\n",
        "# never change comments!\n2 + print\n",
    );
}

#[test]
fn test_trailing_newline() {
    expect_formatting("1 2 *", "1 2 *\n");
    expect_formatting("", "\n");
    expect_formatting("\n", "\n");
    expect_formatting("\n\n", "\n");
    expect_formatting("\n\n\n", "\n");
    expect_formatting("1 2 +\n\n\n", "1 2 +\n");
}

#[test]
fn test_starting_newlines() {
    expect_formatting("\n100 print\n", "100 print\n");
    expect_formatting("\n\n100 print\n", "100 print\n");
    expect_formatting("\n\n\n100 print\n", "100 print\n");
}

#[test]
fn test_indent_block() {
    expect_formatting(
        "\
begin

end
",
        "\
begin

end
",
    );

    expect_formatting(
        "\
begin
1
end
",
        "\
begin
    1
end
",
    );

    expect_formatting(
        "\
begin
begin
begin 1 end
end
end
",
        "\
begin
    begin
        begin 1 end
    end
end
",
    );
}

#[test]
fn test_indent_let() {
    expect_formatting(
        "\
let [  a  b c    ]
a b    +   c  *
end
",
        "\
let [a b c]
    a b + c *
end
",
    );
}

#[test]
fn test_format_float() {
    expect_formatting("1.0\n", "1.0\n");

    expect_formatting("7700000000000000.0\n", "7700000000000000.0\n");

    expect_formatting("123.456\n", "123.456\n");
}

#[test]
fn test_format_deeply_nested() {
    expect_formatting(
        r#"0 -> x
begin
begin
"less" print# less than 5
end
begin
"greater" print# greater than 5
end
x 5 <
if

x 1 + -> x
end
20
dotimes
"#,
        r#"0 -> x
begin
    begin
        "less" print # less than 5
    end
    begin
        "greater" print # greater than 5
    end
    x 5 <
    if

    x 1 + -> x
end
20
dotimes
"#,
    );
}

#[test]
fn test_format_invalid() {
    expect_formatting(
        "\
begin
begin
10",
        "\
begin
    begin
        10
",
    );

    expect_formatting(
        "\
begin
10
end
end
end",
        "\
begin
    10
end
end
end
",
    )
}

#[test]
fn test_format_trailing_spaces() {
    expect_formatting("1 2 +    \t \n", "1 2 +\n");
    expect_formatting("    \t \n", "\n");
    expect_formatting("1    \t \n 2    \n 3  \n", "1\n2\n3\n");
}

#[test]
fn test_format_multiline_string() {
    // NOTE: this string contains trailing spaces
    expect_formatting(
        r#""hello   
test   
1 2 3  
"
"#,
        r#""hello   
test   
1 2 3  
"
"#,
    );
}

#[test]
fn test_string_escape() {
    expect_formatting(
        r#""\t\r\n\0""#,
        r#""\t\r\n\0"
"#,
    )
}

#[test]
fn test_e_notation() {
    expect_formatting(
        "11e3 -40e2 10e-10",
        "11e3 -40e2 10e-10
",
    )
}

fn expect_formatted_file(file: &str, content: &str) -> Result<(), PileError> {
    let test_dir = env!("CARGO_MANIFEST_DIR").to_owned() + "/src/formatting";
    let file = &format!("{}/{}", test_dir, file);

    let program_text = fs::read_to_string(file).unwrap();
    let lexer = Lexer::new(
        &program_text,
        Rc::new(ProgramSource::File(PathBuf::from(file))),
    );
    let result = format(lexer);
    let formatted = fs::read_to_string(file).unwrap();

    if formatted != content {
        eprintln!("===\n{}\n===\n{}\n===", formatted, content);
        assert_eq!(formatted, content);
    }

    result
}

#[test]
fn test_formatting_files() {
    let result = expect_formatted_file(
        "format_a.pile",
        "\
begin
+ # don't touch file!
end
>> # unknown operator
1 2 + print
",
    );
    assert!(result
        .unwrap_err()
        .to_string()
        .ends_with("format_a.pile:4: Unknown operator '>>'"));
}
