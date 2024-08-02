mod elog_logbook;
mod elog_utils;
mod navigate;
mod parser;
use clap::Parser;

use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    logbook_name: String,
    #[arg(long, short)]
    output_file: Option<String>,
    #[arg(long, env)]
    elog_logbook_home: Option<String>,
    #[arg(long, short, num_args = 2)]
    range: Option<Vec<usize>>,
    #[arg(long, short, num_args = 1..)]
    list: Option<Vec<usize>>,
    #[arg(long, short, num_args = 1..)]
    fields: Option<Vec<String>>,
}

fn main() {
    let args = Args::parse();

    // get all of the paths setup
    let logbook_dir = args.elog_logbook_home.expect(
        "Error: no logbook directory could be found. It can be specified with ELOG_LOGBOOK_HOME or at the command line with --elog_logbook_home.",
    );
    let logbook_dir = Path::new(&logbook_dir);

    let book_name = args.logbook_name;

    // find all available log books
    let books = navigate::logbook_hash(logbook_dir);
    let mut logbook = elog_logbook::LogBook::new(&book_name);
    let logs = navigate::list_log_files(books.get(&book_name).unwrap_or_else(|| {
        panic!(
            "No logbook named: {} found in {}",
            book_name,
            logbook_dir.as_os_str().to_str().unwrap()
        )
    }));
    // parse all of the entries.
    logbook.parse_entries(&logs);

    // now get what the user asked for.

    // handle ranges
    let mut keys = match args.range {
        Some(s) => (s[0]..s[1]).map(|x| x as usize).collect(),
        None => vec![],
    };

    // handle selections
    let list_keys = match args.list {
        Some(s) => s,
        None => vec![],
    };

    // check what was given
    let both_args_used = !list_keys.is_empty() & !keys.is_empty();
    let neither_arg_used = list_keys.is_empty() & keys.is_empty();

    if both_args_used {
        panic!("Both range and id list options given, please rerun with only one command.");
    } else if neither_arg_used {
        keys = logbook.entries.keys().copied().collect();
        keys.sort();
    } else if !list_keys.is_empty() {
        keys = list_keys;
    }

    // Now find out what fields are desired
    let fields = match args.fields {
        Some(f) => f,
        None => logbook
            .entries
            .get(&keys[0])
            .unwrap()
            .msg_fields
            .keys()
            .cloned()
            .collect(),
    };

    // make sure everything is capitilized
    let fields: Vec<String> = fields
        .iter()
        .map(|f| elog_utils::uppercase_first_letter(f))
        .collect();

    // utility function for printing
    let field_printer = |vals: &[String]| {
        let mut result: String = "".to_string();
        for val in vals {
            result.push_str(val);
            result.push(',');
        }
        // remove trailing delimiter
        result.pop();
        result
    };

    // print the header
    println!("{}", field_printer(&fields));
    for key in keys {
        let maybe_entry = logbook.entries.get(&key);
        if let Some(e) = maybe_entry {
            let vals: Vec<String> = fields.iter().map(|f| e.get_msg_field(f)).collect();
            println!("{}", field_printer(&vals));
        }
    }
}
