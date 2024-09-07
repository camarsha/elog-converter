use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;

#[derive(Debug)]
pub struct ElogEntry {
    pub entry_id: usize,
    pub entry_dir: PathBuf,
    pub msg_fields: HashMap<String, String>,
    pub attachments: Vec<String>,
}

impl ElogEntry {
    pub fn new(entry_id: usize, entry_dir: PathBuf) -> Self {
        ElogEntry {
            entry_id,
            entry_dir,
            msg_fields: HashMap::new(),
            attachments: vec![],
        }
    }

    pub fn parse(&mut self, file_lines: &[String]) {
        for line in file_lines.into_iter() {
            let line_split: Vec<String> = line.split(": ").map(|ele| ele.to_string()).collect();
            // abort if we have finished all of the key value pairs
            if line_split[0].starts_with("=") {
                break;
            } else {
                // attachments are their own thing
                if line_split[0] == "Attachment" {
                    line_split[1]
                        .split_ascii_whitespace()
                        .for_each(|s| self.attachments.push(s.to_string()));
                } else {
                    self.msg_fields.insert(
                        line_split[0].to_string(),
                        line_split.last().unwrap().to_string(),
                    );
                }
            }
        }
    }

    pub fn get_msg_field(&self, field: &str) -> String {
        let maybe_data = self.msg_fields.get(field);
        let result = match maybe_data {
            Some(d) => d,
            None => "null",
        };
        result.to_string()
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct LogBook {
    name: String,
    pub entries: HashMap<usize, ElogEntry>,
}

impl LogBook {
    pub fn new(name: &str) -> Self {
        LogBook {
            name: name.to_string(),
            entries: HashMap::new(),
        }
    }

    fn get_entry_id(&self, entry_line: &str) -> usize {
        let num_str: usize = entry_line.split(": ").last().unwrap().parse().unwrap();
        num_str
    }

    fn split_entires<'a, 'b>(&'a self, lines: &'b [String]) -> Vec<&'b [String]> {
        let mut start: usize = 0;
        let mut result: Vec<&'b [String]> = Vec::new();
        let mut entry_count = 0;
        for i in 0..lines.len() {
            let is_start_of_entry = lines[i].contains("$@MID@$");
            if is_start_of_entry {
                entry_count += 1
            };
            if is_start_of_entry && i != 0 {
                // push the previous result.
                result.push(&lines[start..i]);
                start = i;
            }
        }
        // handle if there is just one entry and the last entry
        if entry_count == 1 {
            result.push(&lines[0..lines.len()]);
        } else if entry_count >= 1 {
            result.push(&lines[start..lines.len()])
        }
        result
    }

    pub fn parse_entries(&mut self, logbook_files: &[PathBuf]) {
        for file_path in logbook_files.iter() {
            let file = File::open(file_path.as_path())
                .unwrap_or_else(|_| panic!("Unable to open file: {:?}", file_path));
            let reader = BufReader::new(file);
            //            let mut : Vec<String> = Vec::with_capacity(100);
            let all_lines = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();
            // split lines into subvectors that contain the lines for the entries.
            for entry_lines in self.split_entires(&all_lines) {
                // setup the entry
                let id = self.get_entry_id(&entry_lines[0]);
                let mut elog_entry = ElogEntry::new(id, file_path.parent().unwrap().to_path_buf());
                elog_entry.parse(&entry_lines[1..]);
                self.entries.insert(id, elog_entry);
            }
        }
    }
}
