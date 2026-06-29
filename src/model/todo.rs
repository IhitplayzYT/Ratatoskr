pub mod Todo{
    use std::collections::HashSet;

use uuid::Uuid;
    use chrono::{DateTime,Utc};
    use crate::model::meta::Meta::{Tag,Priority}; 

pub struct Todo_task {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: bool,
    pub priority: Priority,
    pub due_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub tags: HashSet<Tag>,
}

impl Todo_task{
    pub fn new(title:String,desc:Option<String>,prio:Option<Priority>,due_date:Option<DateTime<Utc>>,tags:Vec<Tag>) -> Todo_task{
        Self{
            id:Uuid::new_v4(),
            title,
            description:desc,
            status:false,
            priority: if let Some(x) = prio{x} else {Priority::Default},
            due_date,
            created_at:Utc::now(),
            completed_at:None,
            tags:tags.into_iter().collect::<HashSet<Tag>>(),
        }
    }

}


}