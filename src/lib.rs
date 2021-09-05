#![allow(
    unused_assignments,
    unused_imports,
    unused_variables,
    unused_mut,
    dead_code
)]
#[macro_use]
extern crate html5ever;
extern crate markup5ever_rcdom as rcdom;

use std::default::Default;
use std::io;
use std::string::String;

use std::fs::File;

use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use rcdom::{Handle, NodeData, RcDom};
//use html5ever::tree_builder::TreeSink;

mod get_opt_short;
mod log;
#[allow(unused_imports)]
use crate::log::{err, log};

// remove carridge returns
fn minify(string: &str) -> String {
    string.chars().filter(|&c| !"\n".contains(c)).collect()
}

fn remove_newlines(s: &mut str) -> () {
    &mut s.replace("\n", "");
}

fn decompose(ta: &String) -> (&str, &str, &str) {
    match ta {
        t if t.contains("#") => (
            &ta[0..(t.find('#').unwrap())],
            &ta[(t.find('#').unwrap())..=(t.find('#').unwrap())],
            &ta[(t.find('#').unwrap())..],
        ),
        t if t.contains(".") => (
            &ta[0..(t.find('.').unwrap())],
            &ta[(t.find('.').unwrap())..=(t.find('.').unwrap())],
            &ta[(t.find('.').unwrap())..],
        ),
        //t if t.contains(".") => { let t_id: Vec<&str> = t.split(".").collect(); (t_id[0], &("class=".to_owned() + &t_id[1..].join(""))) },
        _ => (ta, "", ""),
    }
}

#[allow(unused_variables)] // we don't always have a parent_tag
fn htmlwalk(
    depth: usize,
    node: &Handle,
    html_vec: &mut Vec<String>,
    attr_vec: &mut Vec<String>,
    verbose: &u8,
    contents: &bool,
    get_attr_value: &bool,
    debug: &u8,
    parent_tag: &mut String,
) {
    let mut get_children = depth;

    //let (mut html_tag, mut attr_type: &str, mut attr);
    let mut html_tag: String;
    let mut attr_type: &str;
    let mut attr: &str;

    // get the next tag from the html_vec
    match html_vec.len() {
        //l if l >= 1 => { let ht = html_vec[0].clone(); { html_tag = format!("{}", ht).as_str(); } },
        l if l >= 1 => {
            html_tag = html_vec[depth].to_string();
            *parent_tag = html_tag.clone();
        },
        _ => html_tag = String::new(),
    };

    match node.data {
        // get the text or inner_html
        NodeData::Text { ref contents } => {
            if get_children >= 1 && &html_tag == parent_tag {
                // Anchor link text {e.g. <a href="#">This part</a>
                // not sure if we want to include implicit carridge returns by default, so for now..
                // NOTE this feature is currently considered unstable, and may change
                if *verbose >= 5 || *debug >= 2 {
                    print!("{}", escape_default(&contents.borrow()));
                }else{
                    // this is currently considered the sane default
                    let formatted_text = escape_default_n(&contents.borrow());
                    if formatted_text.chars().nth(0) == Some(' ') && formatted_text.chars().last() == Some(' ') {
                        //print!("{} [debug:leading&trailing]", escape_default_n(&contents.borrow()));
                        print!("{}", escape_default_n(&contents.borrow()));
                    }else if formatted_text.chars().nth(0) == Some(' '){
                        //print!("{} [debug:t]", escape_default_n(&contents.borrow()));
                        print!("{} ", escape_default_n(&contents.borrow()));
                    }else{
                        //print!(" {}[debug:neither]", escape_default_n(&contents.borrow()));
                        print!(" {}", escape_default_n(&contents.borrow()));
                    }
                }
            }
        },

        // match actual HTML tags
        NodeData::Element {
            ref name,
            ref attrs,
            //ref template_contents,
            ..
        } => {
            // if this node matches the tag that we seek, or it is a child of a node that we seek
             if get_children >= 1
                || name.local.get(0..).unwrap() == html_vec[0]
                || name.local.get(0..).unwrap() == format!("/{}", html_vec[0])
            {

    // if we are at the last tag we have found the depth that was requested
    match html_vec.len() {
        l if l == 1 => {
            get_children += 1;
            *parent_tag = html_vec[0].clone();
        },
        _ => {},
    }
    html_vec.pop();
    if html_tag != "" { log(format!("seeking {:?}", html_tag)); }

                //print!("GC:{} <{}", get_children, name.local.get(0..).unwrap());
                print!("<{}", name.local.get(0..).unwrap());

                // if we aren't looking for the value of a particular attr we dump all attr
                if !get_attr_value {

                    for attr in attrs.borrow().iter() {
                        print!(" {}=\"{}\"", attr.name.local, attr.value);
                    }

                    //print!("<{}", name.local.get(0..).unwrap());

                    // and close the tag
                    match name.local.get(0..).unwrap() {
                        // self closing tag
                        sc if sc == "br".to_string() || sc == "hr" || sc == "img" => print!(" />"),
                        // close tag
                        //ct if ct != "" => print!(">{:?} closed", ct),
                        ct if ct != "" => print!(">"),
                        catch => print!("DEBUG:catch: {:?}", catch),
                    };
                }

            }
        }
        _ => {},
    } // match node.data

    for child in node.children.borrow().iter() {

        let mut child_tags: &mut Vec<String> = html_vec;
        #[rustfmt::skip]
        htmlwalk(get_children, child, child_tags, attr_vec, verbose, contents, get_attr_value, debug, parent_tag);
    }

    match get_children {
        0 | 1 => get_children = 0,
        _ => get_children -=1,
    }


    // add the closing tag, if needed
    match get_children {
        0 if parent_tag != "" => {
            println!("</{}>", parent_tag);
            get_children = 0;
            //parent_tag = &mut String::from("");
        }
        _ => println!("pt: {:?}; GC: {}", parent_tag, get_children),
        //_ => {},
        //_ => println!(" > got_c"),
        //_ => println!("{}", get_children),
    }
    // add new line
}

// Copy of str::escape_default from std, which is currently unstable
pub fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}
// same as escape_default but without carriage return
pub fn escape_default_n(s: &str) -> String {
    s.chars().filter(|&c| !"\n".contains(c)).flat_map(|c| c.escape_default()).collect()
}

//pub fn walk(html_tag: &str, attr: &Option<String>) -> () {
pub fn walk() -> () {
    // gather the flags and arguments
    let mut args = get_opt_short::gos();

    let dom;
    let sink = RcDom::default();

    match args.filename {
        None => {
            let stdin = io::stdin();
            let mut fh = stdin.lock();

            // if rust wasn't so picky about arms matching...
            dom = parse_document(sink, Default::default())
                .from_utf8()
                .read_from(&mut fh)
                .unwrap();
        }
        Some(filename) => {
            // oh its not being piped into stdin
            let mut fh = File::open(filename).unwrap();

            // ... then it would be trivial to re-factor these 4 lines away
            dom = parse_document(sink, Default::default())
                .from_utf8()
                .read_from(&mut fh)
                .unwrap();
        }
    };

    /*
        if args.html_tag == "a.text".to_string() {
            args.html_tag = args.html_tag[0..1].to_string();
            args.attr = Some("text".to_string());
            //args.attr = Some("contents".to_string());
        }

        pub filename: Option<std::path::PathBuf>,
        pub html_tag: Vec<String>,
        pub attr: Vec<String>,
        //pub node: Vec<String>,
        pub verbose: u8,
        pub contents: bool,
        pub get_attr_value: bool,
        pub debug: u8,

    */

    htmlwalk(
        0,
        &dom.document,
        &mut args.html_tag,
        &mut args.attr,
        &args.verbose,
        &args.contents,
        &args.get_attr_value,
        &args.debug,
        &mut "".to_string(),
    );
    if args.verbose > 5 && !dom.errors.is_empty() {
        println!("\nParse errors:");
        for err in dom.errors.iter() {
            println!("    {}", err);
        }
    }
    println!("");
}
//#[path = "./test_lib.rs"]
#[cfg(test)]
mod test_lib;
