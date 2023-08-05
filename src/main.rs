fn main() {
    println!("Hello, world!");
}

#[test]
fn test_reqwest() {
    reqwest::blocking::get("https://google.com").unwrap();
    futures_lite::future::block_on(_not_used);
}

async fn _not_used() {
    let _ = livekit::prelude::Room::connect(&"", &"", livekit::prelude::RoomOptions::default()).await.unwrap();
}
