#[allow(dead_code,non_camel_case_types,non_snake_case)]

pub mod Todo{
    use std::collections::HashSet;

use uuid::Uuid;
    use chrono::{DateTime,Utc};
    use crate::model::meta::Meta::{Tag,Priority}; 

#[derive(Debug,Clone,Hash,PartialEq, Eq)]
pub struct Todo_task {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: bool,
    pub priority: Priority,
    pub due_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub tags: Vec<Tag>,
    pub topic: Option<String>,
    pub member: Option<String>,
}

impl Todo_task{
    pub fn new(title:String,desc:Option<String>,prio:Option<Priority>,due_date:Option<DateTime<Utc>>,tags:Vec<Tag>,topic:Option<String>,member:Option<String>) -> Todo_task{
        Self{
            id:Uuid::new_v4(),
            title,
            description:desc,
            status:false,
            priority: if let Some(x) = prio{x} else {Priority::Default},
            due_date,
            created_at:Utc::now(),
            completed_at:None,
            tags,
            topic,
            member
        }
    }

        pub fn update(&mut self,title:Option<String>,desc: Option<String>,status:bool,prio: Priority,due: Option<DateTime<Utc>>,tags: Option<Vec<Tag>>,member:Option<String>,topic:Option<String>){
            if let Some(x) = title{
                self.title = x;
            }
            self.description = desc;
            self.status = status;
            self.priority = prio;
            self.due_date = due;
            if let Some(x) = tags{
            self.add_tags(x);
            }
            self.member = member;
            self.topic = topic;
            self.created_at = Utc::now();
        }

        pub fn add_tag(&mut self,t:Tag) {
            self.tags.push(t);
        }
        pub fn add_tags(&mut self,t:Vec<Tag>) {
            t.into_iter().for_each(|y| {self.tags.push(y);});
        }


}


}