#![allow(unused_assignments, unused_imports)]
use crate::log::{err, log, usage, warn};
use std::fs;
use std::io::Read;
//use std::convert::TryInto; //needed .parse() not .try_into()

#[derive(Debug)]
pub struct Cli {
    pub filename: Option<std::path::PathBuf>,
    pub html_tag: Vec<String>,
    pub attr: Vec<String>,
    //pub node: Vec<String>,
    pub verbose: u8,
    pub contents: bool,
    pub get_attr_value: bool,
    pub debug: u8,
}

fn display_help() -> () {
    usage("htmlwalk - an html DOM parser

usage: htmlwalk <file.html> [ tag[.class] || tag[#id] || .class || #id ] [ attr=[\"value\"] || text ]
or: cat index.html | htmlwalk [ tag[.class] || tag[#id] || .class || #id ] [ attr=[\"value\"] || text ]

e.g.
    htmlwalk index.html a href => [https://example.com/, https://example.com/menu, ... ]
    htmlwalk index.html img.icon src => https://example.com/img/icon.png
    htmlwalk index.html a.heder href= => [https://example.com/about, https://example.com/contact, /sitemap, ... ]
    htmlwalk index.html div#footer a#contact txt => Contact
    htmlwalk index.html name=\"sought\"  [<a name=\"sought\" href=\"/sought_link/\">You want this!</a>, <div name=\"sought\">foo</div>, ... ]


    # it should be possible to drill down into a tag-within-a-tag
    htmlwalk index.html div#about a href => /about.html
");
    std::process::exit(0);
}

#[allow(dead_code)]
// check if file exists
pub fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

pub fn str_contains(s: &str, c: char) -> bool {
    let sv = &s.chars().collect::<Vec<_>>();
    if sv.contains(&c) {
        true
    } else {
        false
    }
}
/*
 match args {
    "attr=\"values\"" | "attr=values"  => its an attr
    "attr=" | name | value | alt | src => We want the attr value not the entire tag
    ".class" => its a class attr
    "#id" => its an id attr
    "text" | "contents" | "string" | "inner_html" | "innerhtml" => flag to indicate that we just want the ${anchor_link_text} or <img src="x.jpg">This part</img>
    // text obvious, but collision with <textarea>
    // string used by BeautifulSoup
    // contents seems correct
    // inner_html good becaus valid HTML tags don't have underscores, but seems like an obscure incantation from the firebug browser interface
    // innerhtml its easy to forget the underscore, even if that it the only thing ensuring that it isn't an HTML tag
    is_a_file(_) => its a file (we can't use "anything.html" | "foo.htm" because those could be CSS classes )
    tag.class => its a compound tag.class
    tag.class.other_class => its a compound that MUST match both classes
    tag#id => its a compound tag with ID
    #id tag => find the elements with #id and then extract the child element "tag"
    tag1 tag2 tag3 => extract tag3 from withing tag2 from within tag1
    -h => usage && exit
    -v => increment verbose level
    --version | -V => version && exit
    -d => increment debug level
    -f this.html | -f=that.html | --f="other.html" => use this file; Option<["-", "--", "=", '"']>
    _ => its an HTML tag
}

*/

pub fn gos() -> Cli {
    let bin_name = std::env::args().nth(0).expect("[e] we're a ghost?");
    let mut filename: String = String::new();
    //let mut filename: std::path::PathBuf;
    //let mut filename: Option<String> = None;
    let mut html_tag: Vec<String> = vec![];
    let mut attr: Vec<String> = vec![];
    // how chatty do we want to be about errors and such?
    let mut verbose: u8 = 0;
    let mut contents: bool = false;
    let mut debug: u8 = 0;

    // do we want the "https://example.com/" from src="https://example.com/" ?
    let mut get_attr_value: bool = false; // by default we want the entire tag

    let mut joined_flag = 0; // indicate that we are -f looking.for.this
    let mut call_help = 0;
    for (i, arg) in std::env::args().enumerate() {
        if joined_flag > 0 {
            joined_flag = 0;
            continue;
        }
        match arg {
            a if a == "href" || a == "src" || a == "name" || a == "value" || a == "alt" => {
                attr.push(a);
                get_attr_value = true;
            }
            a if a == "text"
                || a == "contents"
                || a == "string"
                || a == "inner_html"
                || a == "innerhtml" =>
            {
                contents = true
            }
            a if a != bin_name && path_exists(&a) => filename = a,
            a if a.starts_with('=') => attr.push(a),
            a if a.starts_with('.') => attr.push(format!("class=\"{}\"", &a[1..])),
            a if a.starts_with('#') && a.ends_with('=') => {
                attr.push(format!("id=\"{}\"", &a[1..(a.len() - 1)]));
                get_attr_value = true;
            }
            a if a.starts_with('#') => attr.push(format!("id=\"{}\"", &a[1..])),
            a if a.ends_with('=') => {
                attr.push(format!("{}", &a[0..(a.len() - 1)]));
                get_attr_value = true;
            }
            a if a.starts_with("-") && a != bin_name => {
                //println!("{} is a flag", &a);
                match &a {
                    flag if flag == "-h" => call_help = 1,
                    flag if flag == "-d" => debug += 1, // potential overflow
                    flag if flag == "-v" => verbose += 1, // potential overflow
                    flag if flag.starts_with("-v") => {
                        let v_count: isize = (flag[2..]).parse().unwrap();
                        verbose += v_count as u8;
                    } //  matches 4 in  e.g. -v4
                    flag if flag == "-f"
                        && path_exists(
                            &std::env::args().nth(i + 1).expect("-f <needs_file.html>"),
                        ) =>
                    {
                        filename = std::env::args().nth(i + 1).expect("missing filename");
                        joined_flag = 1;
                    }
                    flag if flag == "-f" => {
                        err("-f <requires.html>");
                        std::process::exit(0);
                    }
                    flag if flag.starts_with("-f=") || flag.starts_with("--f=") => {
                        warn("-f=\"example.html\" not yet implemented; Try without the quotes.");
                        std::process::exit(0);
                    }
                    f => println!("[w] unknown flag: {}", f),
                };
            }
            a if str_contains(&a, '=') => html_tag.push(a),
            a if str_contains(&a, '.') => html_tag.push(a),
            a if str_contains(&a, '#') => html_tag.push(a), // NTS should we split at this point?
            tag if tag != bin_name => html_tag.push(tag), // NOTE we can't find any <htmlwalk> tags!
            arg_zero if arg_zero == bin_name => (),
            catch => println!("[w] Unknown argument/flag: {}", catch),
        }
    }
    #[allow(path_statements)]
    if call_help > 0 {
        display_help;
    };
    // NTS what's a "path_statements" ?

    // get the HTML file
    let mut file: String = String::new();
    match fs::File::open(&filename) {
        Ok(mut fh) => {
            fh.read_to_string(&mut file)
                .expect("[e] failed to read file from file");
        }
        Err(e) => {
            warn(format!("going to presume you are piping the html into stdin with $(cat example.html|htmlwalk a href) [{:?}", e));
        }
    };

    // compose the CLI struct
    let args = Cli {
        filename: match file.len() {
            0 => None,
            _ => Some(std::path::PathBuf::from(filename)),
        },
        html_tag: html_tag,
        attr: attr,
        verbose: verbose,
        contents: contents,
        get_attr_value: get_attr_value,
        debug: debug,
    };
    if verbose >= 4 {
        log(format!("{:?}", args));
    }
    args
}
