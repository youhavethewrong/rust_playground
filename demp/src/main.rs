#![feature(drain_filter)]
extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::io;

fn main() {
    println!("Main screen turn on.");

    let re = Regex::new(r"(?P<cmd>\w+)\s+(?P<name>\w+)?(\s+\w+\s+)?(?P<dept>\w+)?").unwrap();
    let mut data: HashMap<_, _> = HashMap::new();
    data.insert(String::from("Marketing"), vec![String::from("Andrea")]);

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let caps = re.captures(&input);

        match caps {
            Some(ca) => {
                let cmd = &ca["cmd"];
                match cmd.to_lowercase().as_ref() {
                    "add" => {
                        let n = &ca["name"];
                        let d = &ca["dept"];
                        match data.get_mut(d) {
                            Some(k) => {
                                k.push(n.clone().to_string());
                                println!("Added person to existing department!");
                            }
                            None => {
                                data.insert(d.clone().to_string(), vec![n.clone().to_string()]);
                                println!("Added new department!");
                            }
                        }
                    }
                    "remove" => {
                        let n = &ca["name"];
                        let d = &ca["dept"];

                        match data.get_mut(d) {
                            Some(people) => {
                                let new_people =
                                    people.drain_filter(|x| *x != n).collect::<Vec<_>>();
                                data.insert(d.to_string(), new_people);
                                println!("Removed {} from {}!", n, d);
                            }
                            None => (),
                        }
                    }
                    "print" => {
                        println!("{:#?}", data);
                    }
                    "done" => {
                        println!("Done!");
                        break;
                    }
                    _ => (),
                }
            }
            None => {
                println!("No command or data of the form '<Add> <Name> to <Department>' provided.");
            }
        }
    }
}
