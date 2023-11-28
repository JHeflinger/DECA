#![allow(non_snake_case)]
use dioxus::prelude::*;
use std::fs::File;
use std::io::{self, Read};

fn main() {
    // launch the dioxus app in a webview
    dioxus_desktop::launch(PrimitiveData);
}

// define a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "Hello, world!"
        }
    })
}

fn PrimitiveData(cx: Scope) -> Element {
    let default_file_path = "examples/test_data.jason";
    let mut mystr = String::from("Error: default data not found");
    match read_file_contents(default_file_path) {
        Ok(contents) => {
            mystr = contents.clone();
        }
        Err(err) => {
            eprintln!("Error reading file: {}", err);
        }
    }
    cx.render(rsx! {
        div {
            mystr
        }
    })
}

fn read_file_contents(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}