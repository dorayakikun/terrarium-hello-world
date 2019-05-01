#![feature(proc_macro_hygiene)]
extern crate failure;
extern crate graphql_client;
#[macro_use]
extern crate http_guest;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate maud;
extern crate regex;
extern crate serde;
extern crate serde_json;

use failure::Error;
use maud::{Markup, DOCTYPE};
use regex::Regex;

use http_guest::{Request, Response};

type URI = String;

fn root_page() -> Markup {
    html! {
        div {
            h1 { "Repo Viewer" }
        }
    }
}

fn hello_page() -> Markup {
    html! {
      div {
        h1 { "Hello world" }
      }
    }
}

fn server(req: &Request<Vec<u8>>) -> Result<Response<Vec<u8>>, Error> {
    lazy_static! {
        static ref RE: Regex = Regex::new("/hello").expect("create regex");
    }
    let page_markup = match RE.captures(req.uri().path()) {
        None if req.uri().path() == "/" => root_page(),
        Some(captures) => hello_page(),
        _ => {
            return Ok(Response::builder().status(404).body(vec![]).unwrap());
        }
    };

    let body = html! {
            (DOCTYPE)
            head {
                link rel="stylesheet" type="text/css" href="/style.css" {}
            }
            body {
                div style="display: flex;flex-direction: row; align-items: center; margin: 20px;" {
                    (page_markup)
                }
            }
    }.into_string();

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/html; charset=utf-8")
        .body(body.as_bytes().to_owned())?)
}

fn user_entrypoint(req: &Request<Vec<u8>>) -> Response<Vec<u8>> {
    match server(req) {
        Ok(resp) => resp,
        Err(e) => {
            let body = format!("Hello world demo error: {:?}", e);
            Response::builder()
                .status(500)
                .body(body.as_bytes().to_owned())
                .unwrap()
        }
    }
}

guest_app!(user_entrypoint);
