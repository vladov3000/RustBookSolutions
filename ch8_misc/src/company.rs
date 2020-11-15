use std::collections::{BTreeMap, BTreeSet};
use std::io::{self, Write};
use regex::Regex;

pub type Org = BTreeMap<String, BTreeSet<String>>;

fn print_org(org: &Org) {
    if org.is_empty() {
        println!("Company is currently empty.");
        return;
    }

    println!("{:<20} {:<20}\n{:=<41}", "Department", "Name", "");
    for (dep, names) in org {
        for name in names {
            println!("{0: <20} {1: <20}", dep, name);
        }
    }
}

fn add_to_org(org: &mut Org, r: &Regex, s: &str) -> Option<(String, String)> {
    let caps = r.captures(s)?;
    let name = caps.get(1)?.as_str().to_string();
    let dep = caps.get(r.captures_len() - 1)?.as_str().to_string();

    let res = Some((name.clone(), dep.clone()));
    org.entry(dep).or_insert_with(BTreeSet::new).insert(name);

    res
}

pub fn add_from_io() -> Org {
    let mut res = Org::new();
    let reg_long = Regex::new(r"[A|a]dd ([A-Za-z]+) (to )?([A-Za-z]+)").unwrap();
    let reg_short = Regex::new(r"a ([A-Za-z]+) ([A-Za-z]+)").unwrap();

    println!("Enter input in the form \"Add NAME to DEPARTMENT/a NAME DEPARTMENT\", 
             \"Read/r\" to see all the employees added, or \"Quit/q\".");

    loop {
        let mut line = String::new();

        print!(">");

        io::stdout()
            .flush()
            .expect("Failed to flush output.");

        io::stdin().
            read_line(&mut line)
            .expect("Failed to read line.");

        let line = line.trim();
        let mut words = line.split_whitespace();

        let command = match words.next() {
            Some(c) => c,
            None => continue,
        };
        let mut command = String::from(command);
        command.make_ascii_lowercase();

        match &command[..] {
            "q" | "quit" => break,
            "a" | "add" => {
                let reg = if &command[..] == "a" { &reg_short } else { &reg_long };
                match add_to_org(&mut res, reg, &line) {
                    None => println!("Could not parse add command."),
                    Some((name, dep)) => println!("Added {} to {}.", name, dep),
                }
            }
            "r" | "read" => print_org(&res),
            _ => println!("Command \"{}\" not recognized", command),
        };
    }
    println!("FINAL COMPANY:");
    print_org(&res);
    return res;
}
