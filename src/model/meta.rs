pub mod Meta{
    use std::fmt::Display;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
    use colored::Color;


    #[derive(Debug,PartialEq, Eq,Hash,Clone)]
    pub enum Priority {
    Low,
    Default,
    Medium,
    High,
    Critical,
}

impl From<Priority> for String{
    fn from(value: Priority) -> Self {
        match value{
    Priority::Low => {"Low"},
    Priority::Default => {"Default"},
    Priority::Medium => {"Medium"},
    Priority::High => {"High"},
    Priority::Critical => {"Critical"},
        }.to_string()
        
    }
}

impl From<String> for Priority{
    fn from(value: String) -> Self {
       match &value[..]{
    "Low" => Priority::Low,
    "Default" => Priority::Default,
    "Medium" => Priority::Medium,
    "High" => Priority::High,
    "Critical" => Priority::Critical, 
    _ => Priority::Default
       } 
    }

}

impl Display for Priority{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
    Priority::Low => write!(f,"Low"),
    Priority::Default =>  write!(f,"Default"),
    Priority::Medium =>   write!(f,"Medium"),
    Priority::High =>     write!(f,"High"),
    Priority::Critical => write!(f,"Critical")}
    }
}



    #[derive(Debug,PartialEq, Eq,Hash,Clone)]
pub enum Mood {
    Happy,
    Neutral,
    Sad,
    Angry,
    Excited,
    Tired,
}

impl From<String> for Mood{
fn from(value: String) -> Self {
    match &value.to_lowercase()[..]{
        "happy" => Mood::Happy,
        "neutral" => Mood::Neutral,
        "sad" => Mood::Sad,
        "angry" => Mood::Angry,
        "excited" => Mood::Excited,
        "tired" => Mood::Tired,
        _ => Mood::Neutral
    }
}

}

impl From<Mood> for String{
fn from(value: Mood) -> Self {
    match value{
    Mood::Happy => "Happy",
    Mood::Neutral => "Neutral",
    Mood::Sad => "Sad",
    Mood::Angry => "Angry",
    Mood::Excited => "Excited",
    Mood::Tired => "Tired",
    }.to_string()
}

}



    #[derive(Debug,PartialEq, Eq,Hash)]
pub enum Page {
    Dashboard,
    Todos,
    Notes,
    Journal,
    Calendar,
    Search,
    Settings,
}

#[derive(Debug, Clone, PartialEq, Eq,Hash,Deserialize,Serialize)]
pub enum MyColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    RGB(u8,u8,u8)
}

impl MyColor{
    pub fn new(r:u8,g:u8,b:u8) -> Self{
        Self::RGB(r, g, b)
    }

    pub fn rgb_str(&self) -> String{
        match self{
    MyColor::Black => format!("{:02X}{:02X}{:02X}",0,0,0),
    MyColor::Red => format!("{:02X}{:02X}{:02X}",125,0,0),
    MyColor::Green => format!("{:02X}{:02X}{:02X}",0,125,0),
    MyColor::Yellow => format!("{:02X}{:02X}{:02X}",181, 165, 54),
    MyColor::Blue => format!("{:02X}{:02X}{:02X}",0,0,125),
    MyColor::Magenta => format!("{:02X}{:02X}{:02X}",76, 51, 102),
    MyColor::Cyan => format!("{:02X}{:02X}{:02X}",44, 167, 184),
    MyColor::White => format!("{:02X}{:02X}{:02X}",212, 210, 195),
    MyColor::BrightBlack => format!("{:02X}{:02X}{:02X}",33,33,33),
    MyColor::BrightRed => format!("{:02X}{:02X}{:02X}",255,0,0),
    MyColor::BrightGreen => format!("{:02X}{:02X}{:02X}",0,255,0),
    MyColor::BrightYellow => format!("{:02X}{:02X}{:02X}",255,230,0),
    MyColor::BrightBlue => format!("{:02X}{:02X}{:02X}",0,0,255),
    MyColor::BrightMagenta => format!("{:02X}{:02X}{:02X}",123, 0, 255),
    MyColor::BrightCyan => format!("{:02X}{:02X}{:02X}",0, 218, 247),
    MyColor::BrightWhite => format!("{:02X}{:02X}{:02X}",255,255,255),
    MyColor::RGB(r,g,b) => format!("{:02X}{:02X}{:02X}",r,g,b)
        }
    }



}


impl From<MyColor> for Color {
    fn from(c: MyColor) -> Self {
        match c {
            MyColor::Black => Color::Black,
            MyColor::Red => Color::Red,
            MyColor::Green => Color::Green,
            MyColor::Yellow => Color::Yellow,
            MyColor::Blue => Color::Blue,
            MyColor::Magenta => Color::Magenta,
            MyColor::Cyan => Color::Cyan,
            MyColor::White => Color::White,
            MyColor::BrightBlack => Color::BrightBlack,
            MyColor::BrightRed => Color::BrightRed,
            MyColor::BrightGreen => Color::BrightGreen,
            MyColor::BrightYellow => Color::BrightYellow,
            MyColor::BrightBlue => Color::BrightBlue,
            MyColor::BrightMagenta => Color::BrightMagenta,
            MyColor::BrightCyan => Color::BrightCyan,
            MyColor::BrightWhite => Color::BrightWhite,
            MyColor::RGB(r,g ,b ) => Color::TrueColor { r, g, b }
        }
    }
}


    #[derive(Debug,PartialEq, Eq,Hash,Clone)]
pub struct Tag {
    pub name: String,
    pub color: MyColor,
}

impl Tag{
pub fn new(name:String,color:Option<MyColor>)-> Self{
   Self { name, color: if let Some(x) = color {x} else {MyColor::BrightWhite}} 
}
}

#[derive(Debug,PartialEq, Eq,Clone, Copy)]
pub enum Duration{
SECONDS(u8),
MIN(u8),
YEAR(u8),
DECADE(u8),
CENTURY(u8),
MIN5,
MIN10,
MIN15,
MIN20,
MIN25,
MIN30,
MIN35,
MIN40,
MIN45,
MIN50,
MIN55,
HOUR1,
HOUR2,
HOUR3,
HOUR4,
HOUR5,
HOUR6,
HOUR7,
HOUR8,
HOUR9,
HOUR10,
HOUR11,
HOUR12,
HOUR13,
HOUR14,
HOUR15,
HOUR16,
HOUR17,
HOUR18,
HOUR19,
HOUR20,
HOUR21,
HOUR22,
HOUR23,
HOUR24,
DAY1,
DAY2,
DAY3,
DAY4,
DAY5,
DAY6,
WEEK1,
WEEK2,
WEEK3,
WEEK4,
MONTH1,
MONTH2,
MONTH3,
MONTH4,
MONTH5,
MONTH6,
MONTH7,
MONTH8,
MONTH9,
MONTH10,
MONTH11,
YEAR1,
YEAR2,
YEAR3,
YEAR4,
YEAR5,
YEAR6,
YEAR7,
YEAR8,
YEAR9,
DECADE1,
DECADE2,
DECADE3,
DECADE4,
DECADE5,
DECADE6,
DECADE7,
DECADE8,
DECADE9,
Custom(u8,u8,u8,u8,u8,u8,u8),
Interval(u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8),
}

#[derive(Debug,PartialEq, Eq,Clone, Copy)]
pub enum Frequency{
MINUTLY,
HOURLY,
DAILY,
WEEKLY,
MONTHLY,
YEARLY,
DECADELY,
CENTURYLY,
MIN(u8),
HOUR(u8),
DAY(u8),
WEEK(u8),
MONTH(u8),
YEAR(u8),
DECADE(u8),
CENTURY(u8),
}


impl From<Duration> for String{
fn from(value: Duration) -> Self {
    match value{
Duration::SECONDS(x) => &format!("SECONDS({x})"),
Duration::MIN(x) => &format!("MIN({x})"),
Duration::YEAR(x) => &format!("YEAR({x})"),
Duration::DECADE(x) => &format!("DECADE({x})"),
Duration::CENTURY(x) => &format!("CENTURY({x})"),
Duration::MIN5 => "MIN5",
Duration::MIN10 => "MIN10",
Duration::MIN15 => "MIN15",
Duration::MIN20 => "MIN20",
Duration::MIN25 => "MIN25",
Duration::MIN30 => "MIN30",
Duration::MIN35 => "MIN35",
Duration::MIN40 => "MIN40",
Duration::MIN45 => "MIN45",
Duration::MIN50 => "MIN50",
Duration::MIN55 => "MIN55",
Duration::HOUR1 => "HOUR1",
Duration::HOUR2 => "HOUR2",
Duration::HOUR3 => "HOUR3",
Duration::HOUR4 => "HOUR4",
Duration::HOUR5 => "HOUR5",
Duration::HOUR6 => "HOUR6",
Duration::HOUR7 => "HOUR7",
Duration::HOUR8 => "HOUR8",
Duration::HOUR9 => "HOUR9",
Duration::HOUR10 => "HOUR10",
Duration::HOUR11 => "HOUR11",
Duration::HOUR12 => "HOUR12",
Duration::HOUR13 => "HOUR13",
Duration::HOUR14 => "HOUR14",
Duration::HOUR15 => "HOUR15",
Duration::HOUR16 => "HOUR16",
Duration::HOUR17 => "HOUR17",
Duration::HOUR18 => "HOUR18",
Duration::HOUR19 => "HOUR19",
Duration::HOUR20 => "HOUR20",
Duration::HOUR21 => "HOUR21",
Duration::HOUR22 => "HOUR22",
Duration::HOUR23 => "HOUR23",
Duration::HOUR24 => "HOUR24",
Duration::DAY1 => "DAY1",
Duration::DAY2 => "DAY2",
Duration::DAY3 => "DAY3",
Duration::DAY4 => "DAY4",
Duration::DAY5 => "DAY5",
Duration::DAY6 => "DAY6",
Duration::WEEK1 => "WEEK1",
Duration::WEEK2 => "WEEK2",
Duration::WEEK3 => "WEEK3",
Duration::WEEK4 => "WEEK4",
Duration::MONTH1 => "MONTH1",
Duration::MONTH2 => "MONTH2",
Duration::MONTH3 => "MONTH3",
Duration::MONTH4 => "MONTH4",
Duration::MONTH5 => "MONTH5",
Duration::MONTH6 => "MONTH6",
Duration::MONTH7 => "MONTH7",
Duration::MONTH8 => "MONTH8",
Duration::MONTH9 => "MONTH9",
Duration::MONTH10 => "MONTH10",
Duration::MONTH11 => "MONTH11",
Duration::YEAR1 => "YEAR1",
Duration::YEAR2 => "YEAR2",
Duration::YEAR3 => "YEAR3",
Duration::YEAR4 => "YEAR4",
Duration::YEAR5 => "YEAR5",
Duration::YEAR6 => "YEAR6",
Duration::YEAR7 => "YEAR7",
Duration::YEAR8 => "YEAR8",
Duration::YEAR9 => "YEAR9",
Duration::DECADE1 => "DECADE1",
Duration::DECADE2 => "DECADE2",
Duration::DECADE3 => "DECADE3",
Duration::DECADE4 => "DECADE4",
Duration::DECADE5 => "DECADE5",
Duration::DECADE6 => "DECADE6",
Duration::DECADE7 => "DECADE7",
Duration::DECADE8 => "DECADE8",
Duration::DECADE9 => "DECADE9",
Duration::Custom(a,b,c,d,e,f,g) => &format!("Custom({a},{b},{c},{d},{e},{f},{g})"),
Duration::Interval(a,b,c,d,e,f,g,A,B,C,D,E,F,G ) => &format!("Interval(Custom({a},{b},{c},{d},{e},{f},{g}) -> Custom({A},{B},{C},{D},{E},{F},{G}))"),
    }.to_string()
}

}

}