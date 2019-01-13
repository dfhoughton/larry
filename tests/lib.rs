use std::fs;
use std::iter;
use std::path::Path;
extern crate larry;
use larry::Larry;

fn generic_test(name: &str, lines: &[&str]) {
    fs::write(name, lines.join("")).expect("could not write file");
    if let Ok(mut larry) = Larry::new(Path::new(name)) {
        assert_eq!(lines.len(), larry.len(), "counted lines in file");
        for (i, line) in lines.iter().enumerate() {
            if let Ok(l) = larry.get(i) {
                assert_eq!(*line, l.as_str());
            } else {
                assert!(false, "could not get line");
            }
        }
        if let Err(_) = larry.get(lines.len() + 1) {
            assert!(true, "cannot read past end of file");
        } else {
            assert!(false, "read past end of file");
        }
    } else {
        assert!(false, "cannot create Larry")
    }
    fs::remove_file(name).expect("could not delete file");
}

#[test]
fn simple_list_of_numbers() {
    let lines = ["1\n", "2\n", "3\n", "4\n", "5\n"];
    generic_test("simple_list_of_numbers", &lines);
}

#[test]
fn offset() {
    let lines = ["1\n", "2\n", "3\n", "4\n", "5\n"];
    fs::write("offset", lines.join("")).expect("could not write file");
    if let Ok(larry) = Larry::new(Path::new("offset")) {
        assert_eq!(8, larry.offset(4).unwrap());
    } else {
        assert!(false, "cannot create Larry");
    }
    fs::remove_file("offset").expect("could not delete file");
}

#[test]
fn no_terminal_endline() {
    let lines = ["1\n", "5"];
    generic_test("no_terminal_endline", &lines);
}

#[test]
fn middle_blank_line() {
    let lines = ["1\n", "\n", "5"];
    generic_test("middle_blank_line", &lines);
}

#[test]
fn initial_blank_line() {
    let lines = ["\n", "5"];
    generic_test("initial_blank_line", &lines);
}

#[test]
fn final_blank_line() {
    let lines = ["1\n", "\n"];
    generic_test("final_blank_line", &lines);
}

#[test]
fn understands_r() {
    let lines = ["\r", "1\r", "\r"];
    generic_test("understands_r", &lines);
}

#[test]
fn understands_rn() {
    let lines = ["\r\n", "1\r\n", "\r\n"];
    generic_test("understands_rn", &lines);
}

#[test]
fn understands_nr() {
    let lines = ["\n\r", "1\n\r", "\n\r"];
    generic_test("understands_nr", &lines);
}

#[test]
fn mixed_line_endings() {
    let lines = [
        "1\n", "2\r", "3\r\n", "4\n\r", "space\r", "\r", "space\n", "\n", "\n\r", "\r\n",
    ];
    generic_test("mixed_line_endings", &lines);
}

#[test]
fn large_file() {
    let lines = iter::repeat("these are the times that try men's souls\n")
        .take(50_000)
        .collect::<Vec<&str>>();
    generic_test("large_file", &lines);
}
