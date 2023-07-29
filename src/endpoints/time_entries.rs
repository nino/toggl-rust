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
