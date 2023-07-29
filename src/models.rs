use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DurationMilliSeconds, DurationSeconds};

#[derive(Debug, Serialize, Deserialize)]
pub struct Client {
    /// IsArchived is true if the client is archived
    pub archived: bool,

    /// When was the last update
    pub at: DateTime<Utc>,

    /// Client ID
    pub id: i32,

    /// Name of the client
    pub name: String,

    /// When was deleted, null if not deleted
    pub server_deleted_at: Option<DateTime<Utc>>,

    /// Workspace ID
    pub wid: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub active: bool,
    pub actual_hours: Option<i32>,
    pub at: DateTime<Utc>,
    pub auto_estimates: Option<bool>,
    pub billable: Option<bool>,
    pub cid: i32,
    pub client_id: Option<i32>,
    pub color: String,
    pub created_at: DateTime<Utc>,
    pub currency: Option<String>,
    pub current_period: RecurringPeriod,
    pub end_date: DateTime<Utc>,
    pub estimated_hours: Option<i32>,
    pub first_time_entry: DateTime<Utc>,
    pub fixed_fee: f32,
    pub id: i32,
    pub is_private: bool,
    pub name: String,
    pub rate: f32,
    pub rate_last_updated: Option<DateTime<Utc>>,
    pub recurring: bool,
    pub recurring_parameters: Vec<RecurringProjectParameters>,
    pub server_deleted_at: Option<DateTime<Utc>>,
    pub start_date: DateTime<Utc>,
    pub template: Option<bool>,
    pub wid: i32,
    pub workspace_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecurringProjectParameters {
    pub custom_period: Option<i32>,
    pub estimated_seconds: i32,
    pub parameter_end_date: Option<DateTime<Utc>>,
    pub parameter_start_date: DateTime<Utc>,
    pub period: RecurringPeriod,
    pub project_start_date: DateTime<Utc>,
}

// Who knows if this is right
#[derive(Debug, Serialize, Deserialize)]
pub struct RecurringPeriod {
    pub end_date: Option<DateTime<Utc>>,
    pub start_date: DateTime<Utc>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub active: bool,
    pub at: DateTime<Utc>,

    #[serde_as(as = "Option<DurationSeconds<i64>>")]
    pub estimated_seconds: Option<Duration>,
    pub id: i32,
    pub name: String,
    pub project_id: i32,
    pub recurring: bool,
    pub server_deleted_at: Option<DateTime<Utc>>,

    // The name is wrong in the API -- the value is in milliseconds
    #[serde(rename = "tracked_seconds")]
    #[serde_as(as = "DurationMilliSeconds<i64>")]
    pub tracked_time: Duration,
    pub user_id: Option<i32>,
    pub workspace_id: i32,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct TimeEntry {
    /// When was last updated
    pub at: DateTime<Utc>,

    /// Whether the time entry is marked as billable
    pub billable: bool,

    /// Time Entry description, null if not provided at creation/update
    pub description: Option<String>,

    /// Time entry duration. For running entries should be -1 * (Unix start time)
    #[serde_as(as = "DurationSeconds<i64>")]
    pub duration: Duration,

    /// Used to create a TE with a duration but without a stop time, this field is deprecated for GET endpoints where the value will always be true.
    pub duronly: bool,

    /// Time Entry ID
    pub id: i32,

    /// Project ID, legacy field
    pub pid: i32,

    /// Project ID. Can be null if project was not provided or project was later deleted
    pub project_id: Option<i32>,

    /// When was deleted, null if not deleted
    pub server_deleted_at: Option<DateTime<Utc>>,

    /// Start time in UTC
    pub start: DateTime<Utc>,

    /// Stop time in UTC, can be null if it's still running or created with "duration" and "duronly" fields
    pub stop: Option<DateTime<Utc>>,

    /// Tag IDs, null if tags were not provided or were later deleted
    pub tag_ids: Vec<i32>,

    /// Tag names, null if tags were not provided or were later deleted
    pub tags: Vec<String>,

    /// Task ID. Can be null if task was not provided or project was later deleted
    pub task_id: Option<i32>,

    /// Task ID, legacy field
    pub tid: i32,

    /// Time Entry creator ID, legacy field
    pub uid: i32,

    /// Time Entry creator ID
    pub user_id: i32,

    /// Workspace ID, legacy field
    pub wid: i32,

    /// Workspace ID
    pub workspace_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    /// When was created/last modified
    pub at: DateTime<Utc>,

    /// When was deleted
    pub deleted_at: Option<DateTime<Utc>>,

    /// Tag ID
    pub id: i32,

    /// Tag name
    pub name: String,

    /// Workspace ID
    pub workspace_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Workspace {
    pub admin: bool,
    pub api_token: String,
    pub at: DateTime<Utc>,
    pub business_ws: bool,
    pub csv_upload: Option<CsvUpload>,
    pub default_currency: String,
    pub default_hourly_rate: f64,
    pub ical_enabled: bool,
    pub ical_url: String,
    pub id: i32,
    pub logo_url: String,
    pub name: String,
    pub only_admins_may_create_projects: bool,
    pub only_admins_may_create_tags: bool,
    pub only_admins_see_billable_rates: bool,
    pub only_admins_see_team_dashboard: bool,
    pub organization_id: i32,
    pub premium: bool,
    pub profile: i32,
    pub projects_billable_by_default: bool,
    pub rounding: i32,
    pub rounding_minutes: i32,
    pub server_deleted_at: Option<DateTime<Utc>>,
    pub subscription: Option<Subscription>,
    pub suspended_at: Option<DateTime<Utc>>,
    pub te_constraints: Option<TeConstraints>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CsvUpload {
    pub at: DateTime<Utc>,
    pub log_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Feature {
    pub enabled: bool,
    pub feature_id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Organization {
    /// Whether the requester is an admin of the organization
    pub admin: bool,

    /// Organization's last modification date
    pub at: DateTime<Utc>,

    /// Organization's creation date
    pub created_at: DateTime<Utc>,

    /// Organization ID
    pub id: i32,

    pub is_chargify: bool,

    /// Is true when the organization option is_multi_workspace_enabled is set
    pub is_multi_workspace_enabled: bool,

    pub is_unified: bool,

    /// Maximum number of workspaces allowed for the organization
    pub max_workspaces: i32,

    /// Organization Name
    pub name: String,

    /// Whether the requester is a the owner of the organization
    pub owner: bool,

    /// Organization's subscription payment methods. Omitted if empty.
    pub payment_methods: String,

    /// Organization plan ID
    pub pricing_plan_id: i32,

    /// Organization's delete date
    pub server_deleted_at: Option<DateTime<Utc>>,

    /// Whether the organization is currently suspended
    pub suspended_at: Option<DateTime<Utc>>,

    pub trial_info: TrialInfo,

    /// Number of organization users
    pub user_count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrialInfo {
    /// What was the previous plan before the trial
    pub last_pricing_plan_id: Option<i32>,

    /// When the trial payment is due
    pub next_payment_date: Option<DateTime<Utc>>,

    /// Whether the organization's subscription is currently on trial
    pub trial: bool,

    /// When a trial is available for this organization
    pub trial_available: bool,

    /// When the trial ends
    pub trial_end_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackReminder {
    /// Reminder creation time
    pub created_at: DateTime<Utc>,

    /// Frequency of the reminder in days, should be either 1 or 7
    pub frequency: i32,

    /// Groups IDs to send the reminder to
    pub group_ids: Vec<i32>,

    /// Reminder ID
    pub reminder_id: i32,

    /// Threshold is the number of hours after which the reminder will be sent
    pub threshold: i32,

    /// User IDs to send the reminder to
    pub user_ids: Vec<i32>,

    /// Workspace ID
    pub workspace_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    /// Whether the requester is an admin of the organization
    pub admin: bool,

    /// User's avatar URL
    pub avatar_url: String,

    /// Whether the user can edit their email address
    pub can_edit_email: bool,

    /// User's email address
    pub email: String,

    /// User's groups
    pub groups: Vec<Group>,

    /// User ID
    pub id: i32,

    /// Whether the user is inactive
    pub inactive: bool,

    /// User's invitation code
    pub invitation_code: String,

    /// Whether the user has joined the organization
    pub joined: bool,

    /// User's name
    pub name: String,

    /// Whether the requester is the owner of the organization
    pub owner: bool,

    /// User ID
    pub user_id: i32,

    /// User's workspaces
    pub workspaces: Vec<Workspace>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardDetails {
    // Toggl doesn't seem to document this type
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactDetail {
    // Toggl doesn't seem to document this type
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentDetail {
    // Toggl doesn't seem to document this type
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Period {
    // Toggl doesn't seem to document this type
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subscription {
    pub auto_renew: bool,
    pub card_details: CardDetails,
    pub company_id: i64,
    pub contact_detail: ContactDetail,
    pub created_at: DateTime<Utc>,
    pub currency: String,
    pub customer_id: i64,
    pub deleted_at: Option<DateTime<Utc>>,
    pub last_pricing_plan_id: i64,
    pub organization_id: i64,
    pub payment_details: PaymentDetail,
    pub pricing_plan_id: i64,
    pub renewal_at: DateTime<Utc>,
    pub subscription_id: i64,
    pub subscription_period: Period,
    pub workspace_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeConstraints {
    pub description_present: bool,
    pub project_present: bool,
    pub tag_present: bool,
    pub task_present: bool,
    pub time_entry_constraints_enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
    pub at: DateTime<Utc>,
    pub group_id: i32,
    pub name: String,
    pub users: Vec<User>,
    pub workspaces: Vec<i32>,
}
