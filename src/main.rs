mod client;
mod day_of_week;
mod endpoints;
mod models;
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

    let result = client.get_current_time_entry().await;
    println!("{:#?}", result);

    // Get all projects
    let projects_result = client
        .get_projects(endpoints::get_projects::RequestBody::default())
        .await;
    println!("{:#?}", projects_result);
}
