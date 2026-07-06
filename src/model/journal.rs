#[allow(dead_code,non_camel_case_types,non_snake_case)]

pub mod Journal {
    use std::{collections::HashSet};
    

use uuid::Uuid; 
    use chrono::{DateTime,Utc};
    use crate::model::meta::Meta::{Mood,Tag};

    #[derive(Clone,Debug,PartialEq, Eq,Hash)]
    pub struct Journal_task {
        pub id: Uuid,
        pub title: String,
        pub content: String,
        pub mood: Option<Mood>,
        pub created_at: DateTime<Utc>,
        pub updated_at: DateTime<Utc>,
        pub tags: Vec<Tag>,
        pub member: Option<String>, // School
        pub topic:Option<String>,  // education 
    }

    impl Journal_task{
        pub fn new(title:Option<String>,content: Option<String>,mood:Option<Mood>,tags:Option<Vec<Tag>>,part_of: Option<String>,topic:Option<String>) -> Journal_task{
            Self { id:Uuid::new_v4(), title:if let Some(x) = title{x} else {"Untitled".to_string()}, content: if let Some(x) = content{x} else {"".to_string()}, mood, created_at: Utc::now(), updated_at: Utc::now(), tags: if let Some(x) = tags {x} else {Vec::new()},member:part_of,topic}
        }
        pub fn update(&mut self,title:Option<String>,content: Option<String>,mood: Option<Mood>,tags: Option<Vec<Tag>>,member:Option<String>,topic:Option<String>){
            if let Some(x) = title{
                self.title = x;
            }
            if let Some(x) = content{
                            self.content = x;
                        }
            self.mood = mood;
            if let Some(x) = tags{
            self.add_tags(x);
            }
            self.member = member;
            self.topic = topic;
            self.updated_at = Utc::now();
        }

        pub fn add_tag(&mut self,t:Tag) {
            self.tags.push(t);
            self.updated_at = Utc::now();
        }
        pub fn add_tags(&mut self,t:Vec<Tag>) {
            t.into_iter().for_each(|y| {self.tags.push(y);});
            self.updated_at = Utc::now();
        }
    }
}
