mod client;
mod day_of_week;
mod models;
mod endpoints {
    pub mod me;
    pub mod time_entries;
}
use client::{Auth, Client};

#[tokio::main]
async fn main() {
    let client = Client::new(Auth {
        user: std::env::var("TOGGL_USER").expect("Didn't find TOGGL_USER in environment"),
        password: std::env::var("TOGGL_PASSWORD")
            .expect("Didn't find TOGGL_PASSWORD in environment"),
    });

    let result = client.get_me().await;
    println!("{:#?}", result);
}
