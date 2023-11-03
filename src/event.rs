#[derive(Debug)]
pub struct Event {
    pub title: Option<String>,
    pub organizer: Option<String>,
    pub category: Option<String>,
    pub time: Option<String>,
    pub location: Option<String>,
    pub link: Option<String>,
    pub series: Option<String>,
    pub speaker: Option<String>,
    pub detail: Option<String>,
}