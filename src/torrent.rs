// #[macro_use]
// extern crate serde_derive;

#[derive(Serialize, Deserialize, Debug)]
pub struct Requests {
    pub status: String,
    pub response: Response,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub currentPage: u64,
    pub pages: u64,
    pub results: Vec<Torrent>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Torrent {
    pub voteCount: u64,
    pub bounty: u64,
    pub title: String,
    pub year: u64,
}
