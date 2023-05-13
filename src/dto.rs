pub mod get_me {
    use crate::day_of_week::DayOfWeek;
    use crate::models;
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};

    pub const BASE_URL: &str = "https://api.track.toggl.com/api/v9/me";

    #[derive(Debug, Serialize, Deserialize)]
    pub struct RequestBody {
        with_related_data: Option<bool>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ResponseBody {
        api_token: Option<String>,
        at: DateTime<Utc>,
        beginning_of_week: DayOfWeek,
        clients: Option<Vec<models::Client>>,
        country_id: i32,
        created_at: DateTime<Utc>,
        default_workspace_id: i32,
        email: String,
        fullname: String,
        has_password: bool,
        id: i32,
        image_url: String,
        intercom_hash: Option<String>,
        oauth_providers: Vec<String>,
        openid_email: String,
        openid_enabled: bool,
        projects: Option<Vec<models::Project>>,
        tags: Option<Vec<models::Tag>>,
        tasks: Option<Vec<models::Task>>,
        time_entries: Option<Vec<models::TimeEntry>>,
        timezone: String,
        updated_at: DateTime<Utc>,
        workspaces: Option<Vec<models::Workspace>>,
    }
}

pub mod put_me {
    use crate::day_of_week::DayOfWeek;
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct RequestBody {
        beginning_of_week: Option<DayOfWeek>,
        country_id: Option<i32>,
        current_password: Option<String>,
        default_workspace_id: Option<i32>,
        email: Option<String>,
        fullname: Option<String>,
        password: Option<String>,
        timezone: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ResponseBody {
        api_token: Option<String>,
        at: DateTime<Utc>,
        beginning_of_week: DayOfWeek,
        country_id: i32,
        created_at: DateTime<Utc>,
        default_workspace_id: i32,
        email: String,
        fullname: String,
        has_password: bool,
        id: i32,
        image_url: String,
        openid_email: String,
        openid_enabled: bool,
        // options: Option<models::Options>,
        timezone: String,
        updated_at: DateTime<Utc>,
    }
}
