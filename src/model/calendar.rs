#[allow(dead_code,non_camel_case_types,non_snake_case)]
pub mod Calendar{
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::model::meta::Meta::MyColor;

    use crate::model::meta::Meta::{Duration,Frequency};


    
pub struct Calendar_task{
    pub id:Uuid,
    pub event: String,
    pub desc: Option<String>,
    pub duration: Duration,
    pub frequency: Frequency,
    pub date: DateTime<Utc>,
    pub color: MyColor
}

impl Calendar_task{
    pub fn new(event:String,duration:Option<Duration>,desc:Option<String>,frequency: Option<Frequency>,date:DateTime<Utc>,color:Option<MyColor>) -> Self{
        Self {id:Uuid::new_v4(), event, duration:if let Some(x) = duration {x} else{Duration::DAY(1)} , desc: desc, frequency: if let Some(y) =  frequency{y}else {Frequency::Once}, color: color.unwrap_or( MyColor::BrightBlue),date}
    }

    pub fn update(&mut self,event:Option<String>,duration:Option<Duration>,desc:Option<String>,frequency: Option<Frequency>,date:Option<DateTime<Utc>>,color:Option<MyColor>){
        if let Some(x) = event{
            self.event = x;   
        }if let Some(x) = duration{
            self.duration = x;   
        }if let Some(x)  = desc{
        self.desc = Some(x);
        }
        if let Some(x) = date{
            self.date = x;
        }
        if let Some(x) = frequency{
            self.frequency = x;
        }
        if let Some(x) = color{
            self.color = x;
        }
    }



}


}