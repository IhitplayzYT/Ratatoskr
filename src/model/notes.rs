pub mod Note{
    use uuid::Uuid;
    use chrono::{DateTime,Utc};
    use crate::model::meta::Meta::Tag; 
pub struct Note_task {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub pinned: bool,
    pub favorite: bool,
    pub tags: Vec<Tag>,
}
}