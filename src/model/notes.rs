pub mod Note{
    use std::collections::HashSet;

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
    pub tags: HashSet<Tag>,
    pub topic: Option<String>,
    pub member: Option<String>
}

    impl Note_task{
        pub fn new(title:Option<String>,content: Option<String>,pinned: bool,favorite:bool,tags:Option<Vec<Tag>>,part_of: Option<String>,topic:Option<String>) -> Note_task{
            Self { id:Uuid::new_v4(), title:if let Some(x) = title{x} else {"Untitled".to_string()}, content: if let Some(x) = content{x} else {"".to_string()},created_at: Utc::now(), updated_at: Utc::now(),tags: if let Some(x) = tags {x.into_iter().collect::<HashSet<Tag>>()} else {HashSet::new()},member:part_of,topic,pinned,favorite}
        }
        pub fn update(&mut self,title:Option<String>,content: Option<String>,favorite:bool,pinned: bool,tags: Option<Vec<Tag>>,member:Option<String>,topic:Option<String>){
            if let Some(x) = title{
                self.title = x;
            }
            if let Some(x) = content{
                            self.content = x;
                        }
            self.favorite = favorite;
            self.pinned = pinned;

            if let Some(x) = tags{
            self.add_tags(x);
            }
            self.member = member;
            self.topic = topic;
            self.updated_at = Utc::now();
        }

        pub fn add_tag(&mut self,t:Tag) {
            self.tags.insert(t);
            self.updated_at = Utc::now();
        }
        pub fn add_tags(&mut self,t:Vec<Tag>) {
            t.into_iter().for_each(|y| {self.tags.insert(y);});
            self.updated_at = Utc::now();
        }
    }


}