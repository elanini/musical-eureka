#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate irc;
extern crate regex;
extern crate reqwest;

extern crate htmlescape;
use htmlescape::decode_html;

mod torrent;

use irc::client::prelude::*;
use regex::Regex;
use reqwest::header;
use reqwest::header::{COOKIE, SET_COOKIE};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use torrent::*;

fn main() {
    // let somestr = "Trouble &amp; Problem";
    // println!("og {} decoded {}", somestr, decode_html(somestr).unwrap());
    // get_requests();
    let a = parse_announce("Perfect World [2007] [Album] - FLAC / Lossless / WEB - https://redacted.ch/torrents.php?id=589098 / https://redacted.ch/torrents.php?action=download&id=1993466 - electronic,trance,progressive.trance,tech.house,ambient".to_owned());
    println!("{:#?}", a);
}

#[derive(Debug)]
struct Announce {
    artist_album: String,
    year: u64,
    group_id: u64,
    torrent_id: u64,
}

fn parse_announce(announce: String) -> Announce {
    // ([^[]+)\[(\d{4})\] \[[^\]]+\] -[^-]+- http[^ ]+id=(\d+) \/ http[^ ]+id=(\d+) - .*
    let re = Regex::new(
        r"([^\[]+)\[(\d{4})\] \[[^\]]+\] -[^-]+\- http[^ ]+id=(\d+) / http[^ ]+id=(\d+) - .*",
    ).unwrap();
    let caps: Vec<&str> = re.captures(announce.as_ref())
        .unwrap()
        .iter()
        .skip(1)
        .map(|cap| cap.unwrap().as_str())
        .collect::<Vec<&str>>();
    Announce {
        artist_album: caps[0].to_owned(),
        year: caps[1].parse().unwrap(),
        group_id: caps[2].parse().unwrap(),
        torrent_id: caps[3].parse().unwrap(),
    }
}

fn get_past_release_popularity(artist_id: u64) -> f32 {
    // given artist ID, return some amount of snatches?
    // https://redacted.ch/ajax.php?action=artist&id=
    // adjusted?
    1.0
}

fn get_current_release_popularity(group_id: u64) -> f32 {
    // https://redacted.ch/ajax.php?action=torrentgroup&id=
    // given group ID, return same as above for other torrents
    1.0
}

fn parse_requests_response() {
    let mut f = File::open("res.json").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("couldnt read file");
    let req: Requests = serde_json::from_str(&contents).unwrap();
    println!("{:#?}", req);
}

fn run_irc() {
    // We can also load the Config at runtime via Config::load("path/to/config.toml")
    let config = Config {
        nickname: Some("the-irc-crate".to_owned()),
        server: Some("127.0.0.1".to_owned()),
        channels: Some(vec!["#a".to_owned()]),
        port: Some(6667),
        ..Config::default()
    };

    let mut reactor = IrcReactor::new().unwrap();
    let client = reactor.prepare_client_and_connect(&config).unwrap();
    client.identify().unwrap();
    print!("{:?}", client.list_channels());

    // simple echo
    reactor.register_client_with_handler(client, |client, message| {
        let response_target = message.response_target();
        match message.command {
            Command::PING(_, _) => println!("ping"),
            Command::PRIVMSG(_, ref text) => client.send_privmsg("#a", text.clone()).unwrap(),
            _ => println!("unhandled {}", message),
        }
        // print!("{}", message);
        // And here we can do whatever we want with the messages.
        Ok(())
    });

    reactor.run().unwrap();
}

fn get_requests() {
    let mut params = HashMap::new();
    params.insert("username", "aata844");
    params.insert("password", "password");
    let login_client = reqwest::Client::builder()
        .redirect(reqwest::RedirectPolicy::none())
        .build()
        .unwrap();

    let res = login_client
        .post("https://redacted.ch/login.php")
        .form(&params)
        .send()
        .expect("Login failed");

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::COOKIE,
        res.headers()
            .get(SET_COOKIE)
            .expect("Invalid credentials")
            .clone(),
    );
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    let mut response_string = client
        .get("https://redacted.ch/ajax.php")
        // .get("https://httpbin.org/get")
        .query(&[
            ("action", "requests"),
            ("search", "lil wayne trouble & problem"),
            ("show_filled", "on"),
            ("showall", "on"),
        ])
        .send()
        .unwrap();
    println!("{}", response_string.text().unwrap());
}
