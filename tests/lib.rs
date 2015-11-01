extern crate human_name;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn none_if_empty(s: &str) -> Option<String> {
    if s.is_empty() {
        None
    } else {
        Some(s.to_string())
    }
}

fn format(o: Option<String>) -> String {
    match o {
        Some(s) => format!("'{}'", s),
        None => "n/a".to_string()
    }
}

fn stderr_newline() {
    writeln!(&mut std::io::stderr(), "").ok().unwrap();
}

#[test]
fn parsing() {
    let f = File::open("tests/parseable-names.txt").ok().unwrap();
    let reader = BufReader::new(f);

    stderr_newline();

    for line in reader.lines() {
        let line = line.ok().unwrap();

        if line.starts_with("#") || !line.contains("|") { continue }

        let parts: Vec<&str> = line.split("|").collect();
        let input = parts[0];
        let surname = parts[1];
        let given_name = parts[2];
        let middle_names = parts[3];
        let first_initial = parts[4].chars().nth(0).unwrap();
        let middle_initials = parts[5];

        let name = human_name::Name::parse(input);
        assert!(!name.is_none(), "[{}] Could not parse!", input);

        let given_name = none_if_empty(&given_name);
        let middle_names = none_if_empty(&middle_names);
        let middle_initials = none_if_empty(&middle_initials);

        let name = name.unwrap();
        assert!(name.surname == surname,
                "[{}] Expected surname '{}', got '{}'", input, surname, name.surname);
        assert!(name.first_initial == first_initial,
                "[{}] Expected first initial '{}', got '{}'", input, first_initial, name.first_initial);
        assert!(name.given_name == given_name,
                "[{}] Expected given_name {}, got {}", input, &format(given_name), &format(name.given_name));
        assert!(name.middle_names == middle_names,
                "[{}] Expected middle names {}, got {}", input, &format(middle_names), &format(name.middle_names));
        assert!(name.middle_initials == middle_initials,
                "[{}] Expected middle initials {}, got {}", input, &format(middle_initials), &format(name.middle_initials));

        writeln!(&mut std::io::stderr(), "Parsed '{}' as '{}', {} ('{}') {} ({})",
                 input, surname, &format(given_name), first_initial, &format(middle_names), &format(middle_initials)).ok().unwrap();
    }

    stderr_newline();
}

#[test]
fn unparseable() {
    let f = File::open("tests/unparseable-names.txt").ok().unwrap();
    let reader = BufReader::new(f);

    stderr_newline();

    for line in reader.lines() {
        let line = line.ok().unwrap();

        if line.starts_with("#") { continue }

        let result = human_name::Name::parse(&line);
        assert!(result.is_none(), "'Parsed' junk name: '{}' as '{}'", line, result.unwrap().display());

        writeln!(&mut std::io::stderr(), "Correctly discarded '{}'", line).ok().unwrap();
    }

    stderr_newline();
}