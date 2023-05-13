use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DurationMilliSeconds, DurationSeconds};

#[derive(Debug, Serialize, Deserialize)]
pub struct Client {
    archived: bool,
    at: DateTime<Utc>,
    id: i32,
    name: String,
    server_deleted_at: Option<DateTime<Utc>>,
    wid: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    active: bool,
    actual_hours: Option<i32>,
    at: DateTime<Utc>,
    auto_estimates: Option<bool>,
    billable: Option<bool>,
    cid: i32,
    client_id: Option<i32>,
    color: String,
    created_at: DateTime<Utc>,
    currency: Option<String>,
    current_period: RecurringPeriod,
    end_date: DateTime<Utc>,
    estimated_hours: Option<i32>,
    first_time_entry: DateTime<Utc>,
    fixed_fee: f32,
    id: i32,
    is_private: bool,
    name: String,
    rate: f32,
    rate_last_updated: Option<DateTime<Utc>>,
    recurring: bool,
    recurring_parameters: Vec<RecurringProjectParameters>,
    server_deleted_at: Option<DateTime<Utc>>,
    start_date: DateTime<Utc>,
    template: Option<bool>,
    wid: i32,
    workspace_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecurringProjectParameters {
    custom_period: Option<i32>,
    estimated_seconds: i32,
    parameter_end_date: Option<DateTime<Utc>>,
    parameter_start_date: DateTime<Utc>,
    period: RecurringPeriod,
    project_start_date: DateTime<Utc>,
}

// Who knows if this is right
#[derive(Debug, Serialize, Deserialize)]
pub struct RecurringPeriod {
    end_date: Option<DateTime<Utc>>,
    start_date: DateTime<Utc>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    active: bool,
    at: DateTime<Utc>,

    #[serde_as(as = "Option<DurationSeconds<i64>>")]
    estimated_seconds: Option<Duration>,
    id: i32,
    name: String,
    project_id: i32,
    recurring: bool,
    server_deleted_at: Option<DateTime<Utc>>,

    // The name is wrong in the API -- the value is in milliseconds
    #[serde_as(as = "DurationMilliSeconds<i64>")]
    tracked_seconds: Duration,
    user_id: Option<i32>,
    workspace_id: i32,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct TimeEntry {
    /// When was last updated
    at: DateTime<Utc>,

    /// Whether the time entry is marked as billable
    billable: bool,

    /// Time Entry description, null if not provided at creation/update
    description: Option<String>,

    /// Time entry duration. For running entries should be -1 * (Unix start time)
    #[serde_as(as = "DurationSeconds<i64>")]
    duration: Duration,

    /// Used to create a TE with a duration but without a stop time, this field is deprecated for GET endpoints where the value will always be true.
    duronly: bool,

    /// Time Entry ID
    id: i32,

    /// Project ID, legacy field
    pid: i32,

    /// Project ID. Can be null if project was not provided or project was later deleted
    project_id: Option<i32>,

    /// When was deleted, null if not deleted
    server_deleted_at: Option<DateTime<Utc>>,

    /// Start time in UTC
    start: DateTime<Utc>,

    /// Stop time in UTC, can be null if it's still running or created with "duration" and "duronly" fields
    stop: Option<DateTime<Utc>>,

    /// Tag IDs, null if tags were not provided or were later deleted
    tag_ids: Vec<i32>,

    /// Tag names, null if tags were not provided or were later deleted
    tags: Vec<String>,

    /// Task ID. Can be null if task was not provided or project was later deleted
    task_id: Option<i32>,

    /// Task ID, legacy field
    tid: i32,

    /// Time Entry creator ID, legacy field
    uid: i32,

    /// Time Entry creator ID
    user_id: i32,

    /// Workspace ID, legacy field
    wid: i32,

    /// Workspace ID
    workspace_id: i32,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    /// When was created/last modified
    at: DateTime<Utc>,

    /// When was deleted
    deleted_at: Option<DateTime<Utc>>,

    /// Tag ID
    id: i32,

    /// Tag name
    name: String,

    /// Workspace ID
    workspace_id: i32,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct Workspace {
    /// When was created/last modified
    at: DateTime<Utc>,

    /// List of admins, optional
    admins: Vec<i32>,

    /// Default currency, premium feature, optional, only for existing WS, will be 'USD' initially
    default_currency: Option<String>,

    /// The default hourly rate, premium feature, optional, only for existing WS, will be 0.0 initially
    default_hourly_rate: Option<f32>,

    /// The subscription plan for the workspace, deprecated
    initial_pricing_plan: Option<i32>,

    /// Workspace name
    name: String,

    /// Only admins will be able to create projects, optional, only for existing WS, will be false initially
    only_admins_may_create_projects: Option<bool>,

    /// Only admins will be able to create tags, optional, only for existing WS, will be false initially
    only_admins_may_create_tags: Option<bool>,

    /// Whether only admins will be able to see billable rates, premium feature, optional, only for existing WS. Will be false initially
    only_admins_see_billable_rates: Option<bool>,

    /// Only admins will be able to see the team dashboard, optional, only for existing WS, will be false initially
    only_admins_see_team_dashboard: Option<bool>,

    /// Whether projects will be set as billable by default, premium feature, optional, only for existing WS. Will be true initially
    projects_billable_by_default: Option<bool>,

    /// The rate change mode, premium feature, optional, only for existing WS. Can be "start-today", "override-current", "override-all"
    rate_change_mode: Option<String>,

    /// Whether reports should be collapsed by default, optional, only for existing WS, will be true initially
    reports_collapse: Option<bool>,

    /// Default rounding, premium feature, optional, only for existing WS
    rounding: Option<i32>,

    /// Default rounding in minutes, premium feature, optional, only for existing WS
    rounding_minutes: Option<i32>,

    /// Workspace ID
    id: i32,
}
