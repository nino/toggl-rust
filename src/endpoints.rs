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
        country_id: Option<i32>,
        created_at: DateTime<Utc>,
        default_workspace_id: Option<i32>,
        email: String,
        fullname: String,
        has_password: bool,
        id: i32,
        image_url: String,
        intercom_hash: Option<String>,
        oauth_providers: Option<Vec<String>>,
        openid_email: Option<String>,
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

    pub const BASE_URL: &str = "https://api.track.toggl.com/api/v9/me";

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

pub mod get_clients {
    use crate::models;
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};
    use serde_with::{serde_as, TimestampSeconds};

    pub const BASE_URL: &str = "https://api.track.toggl.com/api/v9/me/clients";

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RequestBody {
        #[serde_as(as = "TimestampSeconds<i64>")]
        since: DateTime<Utc>,
    }

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ResponseBody(Vec<models::Client>);
}

pub mod post_close_account {
    pub const BASE_URL: &str = "https://api.track.toggl.com/api/v9/me/close_account";
}

pub mod get_features {
    use crate::models;
    use serde::{Deserialize, Serialize};

    pub const BASE_URL: &str = "https://api.track.toggl.com/api/v9/me/features";

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ResponseBody {
        features: Vec<models::Feature>,
        workspace_id: i32,
    }
}

pub mod get_location {
    use serde::{Deserialize, Serialize};

    pub const BASE_URL: &str = "https://api.track.toggl.com/api/v9/me/location";

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ResponseBody {
        city: Option<String>,
        city_lat_long: Option<String>,
        country_code: Option<String>,
        country_name: Option<String>,
        state: Option<String>,
    }
}

/// Used to check if authentication works
pub mod get_logged {
    pub const BASE_URL: &str = "https://api.track.toggl.com/api/v9/me/logged";
}

pub mod get_orgs {
    use crate::models;
    use serde::{Deserialize, Serialize};

    pub const BASE_URL: &str = "https://api.track.toggl.com/api/v9/me/organizations";

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ResponseBody(Vec<models::Organization>);
}

pub mod get_projects {
    use crate::models;
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};
    use serde_with::{serde_as, TimestampSeconds};

    pub const BASE_URL: &str = "https://api.track.toggl.com/api/v9/me/projects";

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RequestBody {
        /// Include archived projects.
        include_archived: Option<bool>,

        /// Retrieve projects modified since this date using UNIX timestamp, including deleted ones.
        #[serde_as(as = "Option<TimestampSeconds<i64>>")]
        since: Option<DateTime<Utc>>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ResponseBody(Vec<models::Project>);
}

pub mod get_tags {
    use crate::models;
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};
    use serde_with::{serde_as, TimestampSeconds};

    pub const BASE_URL: &str = "https://api.track.toggl.com/api/v9/me/tags";

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RequestBody {
        /// Retrieve tags modified/deleted since this date using UNIX timestamp.
        #[serde_as(as = "Option<TimestampSeconds<i64>>")]
        since: Option<DateTime<Utc>>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ResponseBody(Vec<models::Tag>);
}

pub mod get_tasks {
    use crate::models;
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};
    use serde_with::{serde_as, TimestampSeconds};

    pub const BASE_URL: &str = "https://api.track.toggl.com/api/v9/me/tasks";

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RequestBody {
        /// Retrieve tasks modified/deleted since this date using UNIX timestamp.
        #[serde_as(as = "Option<TimestampSeconds<i64>>")]
        since: Option<DateTime<Utc>>,

        /// Include tasks marked as done
        include_not_active: Option<bool>, // NOTE: Docs say this is a string, but I seriously hope
                                          // that's a typo
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ResponseBody(Vec<models::Task>);
}

pub mod get_track_reminders {
    use crate::models;
    use serde::{Deserialize, Serialize};

    pub const BASE_URL: &str = "https://api.track.toggl.com/api/v9/me/track_reminders";

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ResponseBody {
        track_reminders: Vec<models::TrackReminder>,
    }
}

pub mod get_workspaces {
    use crate::models;
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};
    use serde_with::{serde_as, TimestampSeconds};

    pub const BASE_URL: &str = "https://api.track.toggl.com/api/v9/me/workspaces";

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RequestBody {
        /// Retrieve tasks modified/deleted since this date using UNIX timestamp.
        #[serde_as(as = "Option<TimestampSeconds<i64>>")]
        since: Option<DateTime<Utc>>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ResponseBody(Vec<models::Workspace>);
}

pub mod get_time_entries {
    use crate::models;
    use chrono::{DateTime, NaiveDate, Utc};
    use serde::{Deserialize, Serialize};
    use serde_with::{serde_as, TimestampSeconds};

    pub const BASE_URL: &str = "https://api.track.toggl.com/api/v9/me/time_entries";

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RequestBody {
        /// Get entries modified since this date using UNIX timestamp, including deleted ones.
        #[serde_as(as = "Option<TimestampSeconds<i64>>")]
        since: Option<DateTime<Utc>>,

        /// Get entries with start time, before given date (YYYY-MM-DD) or with time in RFC3339 format.
        #[serde_as(as = "Option<TimestampSeconds<i64>>")]
        before: Option<DateTime<Utc>>,

        /// Get entries with start time, from start_date YYYY-MM-DD or with time in RFC3339 format. To be used with end_date.
        start_date: Option<NaiveDate>,

        /// Get entries with start time, until end_date YYYY-MM-DD or with time in RFC3339 format. To be used with start_date.
        end_date: Option<NaiveDate>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ResponseBody(Vec<models::TimeEntry>);
}

pub mod get_current_time_entry {
    use crate::models;
    use serde::{Deserialize, Serialize};

    pub const BASE_URL: &str = "https://api.track.toggl.com/api/v9/me/time_entries/current";

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ResponseBody(Option<models::TimeEntry>);
}

pub mod post_time_entries {
    use crate::models;
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};
    use serde_with::{serde_as, TimestampSeconds};

    pub const BASE_URL: &str = "https://api.track.toggl.com/api/v9/workspaces/{workspace_id}/time_entries";

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PathParams {
        workspace_id: i64,
    }



    #[derive(Debug, Serialize, Deserialize)]
    pub struct ResponseBody(Option<models::TimeEntry>);
}
