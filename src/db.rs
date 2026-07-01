pub mod Database{
use std::collections::HashSet;

use chrono::{DateTime, NaiveDateTime, Utc};
use mysql::{
    params,
    prelude::*,
    Pool,
    PooledConn,
    TxOpts,
};

use uuid::Uuid;

    use crate::model::journal::Journal::Journal_task;
    use crate::model::meta::Meta::{Mood, MyColor, Tag};
use crate::model::notes::Note::Note_task;
use crate::model::todo::Todo::Todo_task;
pub struct Database {
    pool: Pool,
}

impl Database {

 //-------------------------------------------------Init APIs----------------------------------------------------
    pub fn new(url: &str) -> mysql::Result<Self> {
        Ok(Self {
            pool: Pool::new(url)?,
        })
    }

    fn conn(&self) -> mysql::Result<PooledConn> {
        self.pool.get_conn()
    }

 //-----------------------------------------------------------------------------------------------------


 //-------------------------------------------------Journal APIs----------------------------------------------------
 
// CREATE TABLE Journal_tasks(
//     id CHAR(36) PRIMARY KEY,
//     title TEXT NOT NULL,
//     content LONGTEXT,
//     mood VARCHAR(20),
//     created_at DATETIME NOT NULL,
//     updated_at DATETIME NOT NULL,
//     member VARCHAR(255)
//     topic VARCHAR(225)
// );
// 
// CREATE TABLE tags(
//     id INT AUTO_INCREMENT PRIMARY KEY,
//     name VARCHAR(255) UNIQUE NOT NULL,
//     color VARCHAR(6) NOT NULL
// );
// 
// CREATE TABLE Journal_task_tags(
//     task_id CHAR(36),
//     tag_id INT,
//     PRIMARY KEY(task_id,tag_id),
//     FOREIGN KEY(task_id) REFERENCES Journal_tasks(id) ON DELETE CASCADE,
//     FOREIGN KEY(tag_id) REFERENCES tags(id) ON DELETE CASCADE
// );





pub fn save_journal_task(&self,task: &Journal_task) -> mysql::Result<()>{
    let mut conn = self.conn()?;
    let mut tx = conn.start_transaction(TxOpts::default())?;
   tx.exec_drop(
            r"
            INSERT INTO Journal_tasks(
                id,
                title,
                content,
                mood,
                created_at,
                updated_at,
                member,
                topic
            )
            VALUES(
                :id,
                :title,
                :content,
                :mood,
                :created,
                :updated,
                :member,
                :topic
            )
            ",
            params! {
                "id" => task.id.to_string(),
                "title" => &task.title,
                "content" => &task.content,
                "mood" => task.mood.as_ref().map(|m| format!("{:?}",m)),
                "created" => task.created_at.naive_utc(),
                "updated" => task.updated_at.naive_utc(),
                "member" => &task.member,
                "topic" => &task.topic, 
            },
        )?;

        for tag in &task.tags {
            self.add_tag(&mut tx, tag)?;
            let tag_id: u64 = tx.exec_first(
                "SELECT id FROM tags WHERE name=?",
                (&tag.name,tag.color.rgb_str()),
            )?
            .unwrap();

            tx.exec_drop(
                r"
                INSERT IGNORE INTO Journal_task_tags(
                    task_id,
                    tag_id
                )
                VALUES(
                    ?,
                    ?
                )
                ",
                (
                    task.id.to_string(),
                    tag_id,
                ),
            )?;
        }
        tx.commit()?;
    Ok(())
}

pub fn load_journal_task(&self,jt_id:Uuid) -> mysql::Result<Option<Journal_task>>{
    let mut conn = self.conn()?;
    let mut tx = conn.start_transaction(TxOpts::default())?;
    
    let mut ret = tx.exec_map(r"
    SELECT title,content,mood,created_at,updated_at,member,topic FROM Journal_tasks jt WHERE jt.id = ?;
    ",(jt_id,),|(t,c,m,ca,ua,me,to):(String,String,String,NaiveDateTime,NaiveDateTime,String,String)|{
        let mut k = Journal_task::new(Some(t), Some(c), Some(Mood::from(m)), None, Some(me), Some(to));
        k.created_at = DateTime::<Utc>::from_naive_utc_and_offset(ca, Utc);
        k.updated_at = DateTime::<Utc>::from_naive_utc_and_offset(ua, Utc);
        k
    })?;
    if ret.is_empty(){
        tx.commit()?;
        return Ok(None);
    } 

    if ret.len() != 1 {
        tx.rollback()?;
        eprintln!("Not Possible all Journal Ids have to be UNIQUE!");
        std::process::exit(2);
    }

    ret[0].tags = self.load_tags(&mut tx, "Journal",jt_id)?.into_iter().collect::<HashSet<Tag>>();
tx.commit()?;
Ok(Some(ret[0].clone()))
}

pub fn load_all_journal_task(&self) -> mysql::Result<Vec<Journal_task>>{
    let mut conn = self.conn()?;
    let mut tx = conn.start_transaction(TxOpts::default())?;
    
    let mut ret = tx.exec_map(r"
    SELECT id,title,content,mood,created_at,updated_at,member,topic FROM Journal_tasks;
    ",(),|(id,t,c,m,ca,ua,me,to):(String,String,String,String,NaiveDateTime,NaiveDateTime,String,String)|{
        let mut k = Journal_task::new(Some(t), Some(c), Some(Mood::from(m)), None, Some(me), Some(to));
        k.id = Uuid::parse_str(&id).unwrap();
        k.created_at = DateTime::<Utc>::from_naive_utc_and_offset(ca, Utc);
        k.updated_at = DateTime::<Utc>::from_naive_utc_and_offset(ua, Utc);
        k
    })?;
    for k in &mut ret{
        k.tags = self.load_tags(&mut tx,"Journal",k.id).unwrap().into_iter().collect::<HashSet<Tag>>();
    }


tx.commit()?;
Ok(ret)
}

pub fn find_journals_with_tags(&self,tags:Vec<Tag>) -> mysql::Result<Vec<Journal_task>>{
let mut conn = self.conn()?;
let mut tx = conn.start_transaction(TxOpts::default())?;
let mut ret = vec![];
tags.iter().for_each(|x| {
let mut k = tx.exec_map(
r"
SELECT jt.*
FROM Journal_tasks AS jt
INNER JOIN Journal_task_tags AS jtt
    ON jt.id = jtt.task_id
INNER JOIN tags AS t
    ON jtt.tag_id = t.id
WHERE t.name = ?;
",(&x.name,),|(id,t,c,m,ca,ua,me,to):(String,String,String,String,NaiveDateTime,NaiveDateTime,String,String)|{
        let mut k = Journal_task::new(Some(t), Some(c), Some(Mood::from(m)), None, Some(me), Some(to));
        k.id = Uuid::parse_str(&id).unwrap();
        k.created_at = DateTime::<Utc>::from_naive_utc_and_offset(ca, Utc);
        k.updated_at = DateTime::<Utc>::from_naive_utc_and_offset(ua, Utc);
        k
}).unwrap();
ret.append(&mut k);
});

for k in &mut ret{
    k.tags = self.load_tags(&mut tx, "Journal", k.id).unwrap().into_iter().collect::<HashSet<Tag>>();
}

tx.commit()?;
Ok(ret)

}

pub fn find_journals_with_member(&self,member:String) -> mysql::Result<Vec<Journal_task>>{
let mut conn = self.conn()?;
let mut tx = conn.start_transaction(TxOpts::default())?;
let mut ret = vec![];
let mut k = tx.exec_map(
r"
SELECT * FROM Journal_task t WHERE t.member = ?;
",(&member,),|(id,t,c,m,ca,ua,me,to):(String,String,String,String,NaiveDateTime,NaiveDateTime,String,String)|{
        let mut k = Journal_task::new(Some(t),Some(c), Some(m.into()), None, Some(me),Some(to));
        k.created_at = DateTime::<Utc>::from_naive_utc_and_offset(ca, Utc);
        k.updated_at = DateTime::<Utc>::from_naive_utc_and_offset(ua, Utc);
        k.id = Uuid::parse_str(&id).unwrap();
        k
}).unwrap();
ret.append(&mut k);

for k in &mut ret{
    k.tags = self.load_tags(&mut tx, "Journal", k.id).unwrap().into_iter().collect::<HashSet<Tag>>();
}

tx.commit()?;
Ok(ret)

}
pub fn find_journals_with_topic(&self,topic:String) -> mysql::Result<Vec<Journal_task>>{
let mut conn = self.conn()?;
let mut tx = conn.start_transaction(TxOpts::default())?;
let mut ret = vec![];
let mut k = tx.exec_map(
r"
SELECT * FROM Journal_task t WHERE t.topic = ?;
",(&topic,),|(id,t,c,m,ca,ua,me,to):(String,String,String,String,NaiveDateTime,NaiveDateTime,String,String)|{
        let mut k = Journal_task::new(Some(t),Some(c), Some(m.into()), None, Some(me),Some(to));
        k.created_at = DateTime::<Utc>::from_naive_utc_and_offset(ca, Utc);
        k.updated_at = DateTime::<Utc>::from_naive_utc_and_offset(ua, Utc);
        k.id = Uuid::parse_str(&id).unwrap();
        k
}).unwrap();
ret.append(&mut k);

for k in &mut ret{
    k.tags = self.load_tags(&mut tx, "Journal", k.id).unwrap().into_iter().collect::<HashSet<Tag>>();
}

tx.commit()?;
Ok(ret)

}


 //-----------------------------------------------------------------------------------------------------

 //-------------------------------------------------Generic APIs----------------------------------------------------
pub fn delete_task(&self,table:&str,id:Uuid) -> mysql::Result<()>{
let mut conn = self.conn()?;
let mut tx = conn.start_transaction(TxOpts::default())?; 
let stmt = format!("DELETE FROM {}_task WHERE id = ?",table);
tx.exec_drop(&stmt,(id,))?;
Ok(())
} 


pub fn load_tags(&self,conn: &mut impl Queryable,table:&str,id: Uuid) -> mysql::Result<Vec<Tag>> {
    let stmt = format!(r"
        SELECT t.name.t.color
        FROM tags t
        INNER JOIN {}_task_tags tt
            ON t.id = tt.tag_id
        WHERE jtt.task_id = ?
        ",table
 );
    conn.exec_map(stmt,
        (id.to_string(),),
        |(name,color):(String,String)| {
            let r = u8::from_str_radix(&color[0..2], 16).unwrap();
            let g = u8::from_str_radix(&color[2..4], 16).unwrap();
            let b = u8::from_str_radix(&color[4..6], 16).unwrap();
            Tag::new(name, Some(MyColor::RGB(r, g, b)))},
    )
}

pub fn add_tag(&self,conn:&mut impl Queryable,tag: &Tag) -> mysql::Result<()>{
    conn.exec_drop(r"
    INSERT INTO tags(name,color) VALUES(?.?);
    ", (&tag.name,tag.color.rgb_str(),))?;
    Ok(())
}




//-----------------------------------------------------------------------------------------------------




 //-------------------------------------------------Notes APIs----------------------------------------------------

//    CREATE TABLE Note_tasks(
//        id CHAR(36) PRIMARY KEY,
//        title TEXT NOT NULL,
//        content LONGTEXT,
//        created_at DATETIME NOT NULL,
//        updated_at DATETIME NOT NULL,
//        pinned BOOLEAN,
//        favorite BOOLEAN
//        member VARCHAR(255)
//        topic VARCHAR(225)
//    );
//    
//     CREATE TABLE tags(
//         id INT AUTO_INCREMENT PRIMARY KEY,
//         name VARCHAR(255) UNIQUE NOT NULL,
//         color VARCHAR(6) NOT NULL
//     );
//
//    CREATE TABLE Note_task_tags(
//        task_id CHAR(36),
//        tag_id INT,
//        PRIMARY KEY(task_id,tag_id),
//        FOREIGN KEY(task_id) REFERENCES Note_tasks(id) ON DELETE CASCADE,
//        FOREIGN KEY(tag_id) REFERENCES tags(id) ON DELETE CASCADE
//    );
//    



pub fn save_note_task(&self,task: &Note_task) -> mysql::Result<()>{
    let mut conn = self.conn()?;
    let mut tx = conn.start_transaction(TxOpts::default())?;
   tx.exec_drop(
            r"
            INSERT INTO Note_tasks(
                id,
                title,
                content,
                created_at,
                updated_at,
                pinned,
                favorite,
                member,
                topic
            )
            VALUES(
                :id,
                :title,
                :content,
                :created,
                :updated,
                :pinned,
                :favorite,
                :member,
                :topic
            )
            ",
            params! {
                "id" => task.id.to_string(),
                "title" => &task.title,
                "content" => &task.content,
                "created" => task.created_at.naive_utc(),
                "updated" => task.updated_at.naive_utc(),
                "pinned" => task.pinned,
                "favorite" => task.favorite ,
                "member" => &task.member,
                "topic" => &task.topic, 
            },
        )?;

        for tag in &task.tags {
            self.add_tag(&mut tx, tag)?;
            let tag_id: u64 = tx.exec_first(
                "SELECT id FROM tags WHERE name=?",
                (&tag.name,tag.color.rgb_str()),
            )?
            .unwrap();

            tx.exec_drop(
                r"
                INSERT IGNORE INTO Note_task_tags(
                    task_id,
                    tag_id
                )
                VALUES(
                    ?,
                    ?
                )
                ",
                (
                    task.id.to_string(),
                    tag_id,
                ),
            )?;
        }
        tx.commit()?;
    Ok(())
}

pub fn load_note_task(&self,jt_id:Uuid) -> mysql::Result<Option<Note_task>>{
    let mut conn = self.conn()?;
    let mut tx = conn.start_transaction(TxOpts::default())?;
    
    let mut ret = tx.exec_map(r"
    SELECT title,content,created_at,updated_at,pinned,favorite,member,topic FROM Note_tasks jt WHERE jt.id = ?;
    ",(jt_id,),|(t,c,m,ca,ua,p,f,me,to):(String,String,String,NaiveDateTime,NaiveDateTime,bool,bool,String,String)|{
        let mut k = Note_task::new(Some(t),Some(c), p, f, None, Some(me),Some(to));
        k.created_at = DateTime::<Utc>::from_naive_utc_and_offset(ca, Utc);
        k.updated_at = DateTime::<Utc>::from_naive_utc_and_offset(ua, Utc);
        k
    })?;
    if ret.is_empty(){
        tx.commit()?;
        return Ok(None);
    } 

    if ret.len() != 1 {
        tx.rollback()?;
        eprintln!("Not Possible all Note Ids have to be UNIQUE!");
        std::process::exit(2);
    }

    ret[0].tags = self.load_tags(&mut tx, "Note",jt_id)?.into_iter().collect::<HashSet<Tag>>();
tx.commit()?;
Ok(Some(ret[0].clone()))
}

pub fn load_all_note_task(&self) -> mysql::Result<Vec<Note_task>>{
    let mut conn = self.conn()?;
    let mut tx = conn.start_transaction(TxOpts::default())?;
    
    let mut ret = tx.exec_map(r"
    SELECT id,title,content,created_at,updated_at,pinned,favorite,member,topic FROM Note_tasks;
    ",(),|(id,t,c,m,ca,ua,p,f,me,to):(String,String,String,String,NaiveDateTime,NaiveDateTime,bool,bool,String,String)|{
        let mut k = Note_task::new(Some(t),Some(c), p, f, None, Some(me),Some(to));
        k.created_at = DateTime::<Utc>::from_naive_utc_and_offset(ca, Utc);
        k.updated_at = DateTime::<Utc>::from_naive_utc_and_offset(ua, Utc);
        k.id = Uuid::parse_str(&id).unwrap();
        k
    })?;
    for k in &mut ret{
        k.tags = self.load_tags(&mut tx,"Note",k.id).unwrap().into_iter().collect::<HashSet<Tag>>();
    }


tx.commit()?;
Ok(ret)
}

pub fn find_notes_with_tags(&self,tags:Vec<Tag>) -> mysql::Result<Vec<Note_task>>{
let mut conn = self.conn()?;
let mut tx = conn.start_transaction(TxOpts::default())?;
let mut ret = vec![];
tags.iter().for_each(|x| {
let mut k = tx.exec_map(
r"
SELECT nt.*
FROM Note_tasks AS nt
INNER JOIN Note_task_tags AS ntt
    ON nt.id = ntt.task_id
INNER JOIN tags AS t
    ON ntt.tag_id = t.id
WHERE t.name = ?;
",(&x.name,),|(id,t,c,m,ca,ua,p,f,me,to):(String,String,String,String,NaiveDateTime,NaiveDateTime,bool,bool,String,String)|{
        let mut k = Note_task::new(Some(t),Some(c), p, f, None, Some(me),Some(to));
        k.created_at = DateTime::<Utc>::from_naive_utc_and_offset(ca, Utc);
        k.updated_at = DateTime::<Utc>::from_naive_utc_and_offset(ua, Utc);
        k.id = Uuid::parse_str(&id).unwrap();
        k
}).unwrap();
ret.append(&mut k);
});

for k in &mut ret{
    k.tags = self.load_tags(&mut tx, "Note", k.id).unwrap().into_iter().collect::<HashSet<Tag>>();
}

tx.commit()?;
Ok(ret)

}

pub fn find_notes_with_member(&self,member:String) -> mysql::Result<Vec<Note_task>>{
let mut conn = self.conn()?;
let mut tx = conn.start_transaction(TxOpts::default())?;
let mut ret = vec![];
let mut k = tx.exec_map(
r"
SELECT * FROM Note_task t WHERE t.member = ?;
",(&member,),|(id,t,c,m,ca,ua,p,f,me,to):(String,String,String,String,NaiveDateTime,NaiveDateTime,bool,bool,String,String)|{
        let mut k = Note_task::new(Some(t),Some(c), p, f, None, Some(me),Some(to));
        k.created_at = DateTime::<Utc>::from_naive_utc_and_offset(ca, Utc);
        k.updated_at = DateTime::<Utc>::from_naive_utc_and_offset(ua, Utc);
        k.id = Uuid::parse_str(&id).unwrap();
        k
}).unwrap();
ret.append(&mut k);

for k in &mut ret{
    k.tags = self.load_tags(&mut tx, "Note", k.id).unwrap().into_iter().collect::<HashSet<Tag>>();
}

tx.commit()?;
Ok(ret)

}

pub fn find_notes_with_topic(&self,topic:String) -> mysql::Result<Vec<Note_task>>{
let mut conn = self.conn()?;
let mut tx = conn.start_transaction(TxOpts::default())?;
let mut ret = vec![];
let mut k = tx.exec_map(
r"
SELECT * FROM Note_task t WHERE t.topic = ?;
",(&topic,),|(id,t,c,m,ca,ua,p,f,me,to):(String,String,String,String,NaiveDateTime,NaiveDateTime,bool,bool,String,String)|{
        let mut k = Note_task::new(Some(t),Some(c), p, f, None, Some(me),Some(to));
        k.created_at = DateTime::<Utc>::from_naive_utc_and_offset(ca, Utc);
        k.updated_at = DateTime::<Utc>::from_naive_utc_and_offset(ua, Utc);
        k.id = Uuid::parse_str(&id).unwrap();
        k
}).unwrap();
ret.append(&mut k);

for k in &mut ret{
    k.tags = self.load_tags(&mut tx, "Note", k.id).unwrap().into_iter().collect::<HashSet<Tag>>();
}

tx.commit()?;
Ok(ret)
}



 //-----------------------------------------------------------------------------------------------------


 //-------------------------------------------------Notes APIs----------------------------------------------------

//   CREATE TABLE Todo_tasks(
//      id CHAR(36) PRIMARY KEY,
//      title TEXT NOT NULL,
//      description LONGTEXT,
//      status BOOLEAN,
//      priority VARCHAR(20),
//      due_at DATETIME,
//      created_at DATETIME NOT NULL,
//      completed_at DATETIME,
//      member VARCHAR(255)
//      topic VARCHAR(225)
//  );
// 
//  
//  CREATE TABLE tags(
//      id INT AUTO_INCREMENT PRIMARY KEY,
//      name VARCHAR(255) UNIQUE NOT NULL,
//      color VARCHAR(6) NOT NULL
//  );
//  
//  CREATE TABLE Todo_task_tags(
//      task_id CHAR(36),
//      tag_id INT,
//      PRIMARY KEY(task_id,tag_id),
//      FOREIGN KEY(task_id) REFERENCES Todo_tasks(id) ON DELETE CASCADE,
//      FOREIGN KEY(tag_id) REFERENCES tags(id) ON DELETE CASCADE
//  );
//  

pub fn save_todo_task(&self,task: &Todo_task) -> mysql::Result<()>{
    let mut conn = self.conn()?;
    let mut tx = conn.start_transaction(TxOpts::default())?;
   tx.exec_drop(
            r"
            INSERT INTO Todo_tasks(
                id,
                title,
                description,
                status,
                priority,
                due_at,
                created_at,
                completed_at,
                member,
                topic
            )
            VALUES(
                :id,
                :title,
                :description,
                :status,
                :priority,
                :due_at,
                :created_at,
                :completed_at,
                :member,
                :topic
            )
            ",
            params! {
                "id" => task.id.to_string(),
                "title" => &task.title,
                "description" => &task.description,
                "status" => &task.status,
                "priority" => &format!("{}",task.priority),
                "due_at" => task.due_date.unwrap().naive_utc(),
                "created_at" => task.created_at.naive_utc(),
                "completed_at" => task.completed_at.unwrap().naive_utc(),
                "member" => &task.member,
                "topic" => &task.topic, 
            },
        )?;

        for tag in &task.tags {
            self.add_tag(&mut tx, tag)?;
            let tag_id: u64 = tx.exec_first(
                "SELECT id FROM tags WHERE name=?",
                (&tag.name,tag.color.rgb_str()),
            )?
            .unwrap();

            tx.exec_drop(
                r"
                INSERT IGNORE INTO Todo_task_tags(
                    task_id,
                    tag_id
                )
                VALUES(
                    ?,
                    ?
                )
                ",
                (
                    task.id.to_string(),
                    tag_id,
                ),
            )?;
        }
        tx.commit()?;
    Ok(())
}

pub fn load_todo_task(&self,jt_id:Uuid) -> mysql::Result<Option<Todo_task>>{
    let mut conn = self.conn()?;
    let mut tx = conn.start_transaction(TxOpts::default())?;
    
    let mut ret = tx.exec_map(r"
    SELECT * FROM Todo_tasks jt WHERE jt.id = ?;
    ",(jt_id,),|(id,t,d,s,p,da,ca,cpa,me,to):(String,String,String,bool,String,NaiveDateTime,NaiveDateTime,NaiveDateTime,String,String)|{
        let mut k = Todo_task::new(t, Some(d), Some(p.into()),None,vec![],Some(to),Some(me));
        k.created_at = DateTime::<Utc>::from_naive_utc_and_offset(ca, Utc);
        k.due_date = Some(DateTime::<Utc>::from_naive_utc_and_offset(da, Utc));
        k.completed_at = Some(DateTime::<Utc>::from_naive_utc_and_offset(cpa, Utc));
        k.status = s;
        k.id = Uuid::parse_str(&id).unwrap();
        k
    })?;
    if ret.is_empty(){
        tx.commit()?;
        return Ok(None);
    } 

    if ret.len() != 1 {
        tx.rollback()?;
        eprintln!("Not Possible all Todo Ids have to be UNIQUE!");
        std::process::exit(2);
    }

    ret[0].tags = self.load_tags(&mut tx, "Todo",jt_id)?.into_iter().collect::<HashSet<Tag>>();
tx.commit()?;
Ok(Some(ret[0].clone()))
}

pub fn load_all_todo_task(&self) -> mysql::Result<Vec<Todo_task>>{
    let mut conn = self.conn()?;
    let mut tx = conn.start_transaction(TxOpts::default())?;
    
    let mut ret = tx.exec_map(r"
    SELECT * FROM Todo_tasks;
    ",(),|(id,t,d,s,p,da,ca,cpa,me,to):(String,String,String,bool,String,NaiveDateTime,NaiveDateTime,NaiveDateTime,String,String)|{
        let mut k = Todo_task::new(t, Some(d), Some(p.into()),None,vec![],Some(to),Some(me));
        k.created_at = DateTime::<Utc>::from_naive_utc_and_offset(ca, Utc);
        k.due_date = Some(DateTime::<Utc>::from_naive_utc_and_offset(da, Utc));
        k.completed_at = Some(DateTime::<Utc>::from_naive_utc_and_offset(cpa, Utc));
        k.status = s;
        k.id = Uuid::parse_str(&id).unwrap();
        k
    })?;
    for k in &mut ret{
        k.tags = self.load_tags(&mut tx,"Todo",k.id).unwrap().into_iter().collect::<HashSet<Tag>>();
    }

tx.commit()?;
Ok(ret)
}

pub fn find_todo_with_tags(&self,tags:Vec<Tag>) -> mysql::Result<Vec<Todo_task>>{
let mut conn = self.conn()?;
let mut tx = conn.start_transaction(TxOpts::default())?;
let mut ret = vec![];
tags.iter().for_each(|x| {
let mut k = tx.exec_map(
r"
SELECT tt.*
FROM Todo_tasks AS tt
INNER JOIN Todo_task_tags AS ttt
    ON tt.id = ttt.task_id
INNER JOIN tags AS t
    ON ttt.tag_id = t.id
WHERE t.name = ?;
",(&x.name,),|(id,t,d,s,p,da,ca,cpa,me,to):(String,String,String,bool,String,NaiveDateTime,NaiveDateTime,NaiveDateTime,String,String)|{
        let mut k = Todo_task::new(t, Some(d), Some(p.into()),None,vec![],Some(to),Some(me));
        k.created_at = DateTime::<Utc>::from_naive_utc_and_offset(ca, Utc);
        k.due_date = Some(DateTime::<Utc>::from_naive_utc_and_offset(da, Utc));
        k.completed_at = Some(DateTime::<Utc>::from_naive_utc_and_offset(cpa, Utc));
        k.status = s;
        k.id = Uuid::parse_str(&id).unwrap();
        k
}).unwrap();
ret.append(&mut k);
});

for k in &mut ret{
    k.tags = self.load_tags(&mut tx, "Todo", k.id).unwrap().into_iter().collect::<HashSet<Tag>>();
}

tx.commit()?;
Ok(ret)

}

pub fn find_todos_with_member(&self,member:String) -> mysql::Result<Vec<Todo_task>>{
let mut conn = self.conn()?;
let mut tx = conn.start_transaction(TxOpts::default())?;
let mut ret = vec![];
let mut k = tx.exec_map(
r"
SELECT * FROM Todo_task t WHERE t.member = ?;
",(&member,),|(id,t,d,s,p,da,ca,cpa,me,to):(String,String,String,bool,String,NaiveDateTime,NaiveDateTime,NaiveDateTime,String,String)|{
        let mut k = Todo_task::new(t, Some(d), Some(p.into()),None,vec![],Some(to),Some(me));
        k.created_at = DateTime::<Utc>::from_naive_utc_and_offset(ca, Utc);
        k.due_date = Some(DateTime::<Utc>::from_naive_utc_and_offset(da, Utc));
        k.completed_at = Some(DateTime::<Utc>::from_naive_utc_and_offset(cpa, Utc));
        k.status = s;
        k.id = Uuid::parse_str(&id).unwrap();
        k
}).unwrap();
ret.append(&mut k);

for k in &mut ret{
    k.tags = self.load_tags(&mut tx, "Todo", k.id).unwrap().into_iter().collect::<HashSet<Tag>>();
}

tx.commit()?;
Ok(ret)

}

pub fn find_todos_with_topic(&self,topic:String) -> mysql::Result<Vec<Todo_task>>{
let mut conn = self.conn()?;
let mut tx = conn.start_transaction(TxOpts::default())?;
let mut ret = vec![];
let mut k = tx.exec_map(
r"
SELECT * FROM Todo_task t WHERE t.topic = ?;
",(&topic,),|(id,t,d,s,p,da,ca,cpa,me,to):(String,String,String,bool,String,NaiveDateTime,NaiveDateTime,NaiveDateTime,String,String)|{
        let mut k = Todo_task::new(t, Some(d), Some(p.into()),None,vec![],Some(to),Some(me));
        k.created_at = DateTime::<Utc>::from_naive_utc_and_offset(ca, Utc);
        k.due_date = Some(DateTime::<Utc>::from_naive_utc_and_offset(da, Utc));
        k.completed_at = Some(DateTime::<Utc>::from_naive_utc_and_offset(cpa, Utc));
        k.status = s;
        k.id = Uuid::parse_str(&id).unwrap();
        k
}).unwrap();
ret.append(&mut k);

for k in &mut ret{
    k.tags = self.load_tags(&mut tx, "Todo", k.id).unwrap().into_iter().collect::<HashSet<Tag>>();
}

tx.commit()?;
Ok(ret)
}

 //-----------------------------------------------------------------------------------------------------

}


}