#[allow(dead_code,non_camel_case_types,non_snake_case)]

pub mod Meta{
use std::fmt::Display;
use ratatui::style::Color;
use serde::{Deserialize, Serialize};

use crate::model::app::App::Color_channel;


#[derive(Debug,PartialEq, Eq,Hash,Clone,Copy,PartialOrd)]
    pub enum Priority {
    Low,
    Default,
    Medium,
    High,
    Critical,
    }

impl Priority {
    const ALL: [Priority; 5] = [Priority::Low, Priority::Default, Priority::Medium, Priority::High, Priority::Critical];
    pub fn title(&self) -> &'static str {
        match self {
            Priority::Low => "Low", Priority::Default => "Default", Priority::Medium => "Medium",
            Priority::High => "High", Priority::Critical => "Critical",
        }
    }
    pub fn next(self) -> Self { let i = Self::ALL.iter().position(|p| *p == self).unwrap(); Self::ALL[(i + 1) % Self::ALL.len()] }
    pub fn prev(self) -> Self { let i = Self::ALL.iter().position(|p| *p == self).unwrap(); Self::ALL[(i + Self::ALL.len() - 1) % Self::ALL.len()] }
    pub fn color(&self) -> Color {
        match self {
            Priority::Low => Color::Gray, Priority::Default => Color::White, Priority::Medium => Color::Yellow,
            Priority::High => Color::LightRed, Priority::Critical => Color::Red,
        }
    }
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



    #[derive(Debug,PartialEq, Eq,Hash,Clone,Copy)]
pub enum Mood {
    Happy,
    Neutral,
    Sad,
    Angry,
    Excited,
    Tired,
}

impl Mood {
    pub const ALL: [Mood; 6] = [Mood::Happy, Mood::Neutral, Mood::Sad, Mood::Angry, Mood::Excited, Mood::Tired];
    pub fn title(&self) -> &'static str {
        match self {
            Mood::Happy => "Happy",
            Mood::Neutral => "Neutral",
            Mood::Sad => "Sad",
            Mood::Angry => "Angry",
            Mood::Excited => "Excited",
            Mood::Tired => "Tired",
        }
    }
    pub fn next(self) -> Self { let i = Self::ALL.iter().position(|m| *m == self).unwrap(); Self::ALL[(i + 1) % Self::ALL.len()] }
    pub fn prev(self) -> Self { let i = Self::ALL.iter().position(|m| *m == self).unwrap(); Self::ALL[(i + Self::ALL.len() - 1) % Self::ALL.len()] }
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



#[derive(Debug, Clone,Copy,PartialEq, Eq,Hash,Deserialize,Serialize)]
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

    pub fn get_rgb(&self) -> (u8,u8,u8){
        let (r, g, b) = match *self {
            MyColor::Black => (0, 0, 0),
            MyColor::Red => (125, 0, 0),
            MyColor::Green => (0, 125, 0),
            MyColor::Yellow => (181, 165, 54),
            MyColor::Blue => (0, 0, 125),
            MyColor::Magenta => (76, 51, 102),
            MyColor::Cyan => (44, 167, 184),
            MyColor::White => (212, 210, 195),
            MyColor::BrightBlack => (33, 33, 33),
            MyColor::BrightRed => (255, 0, 0),
            MyColor::BrightGreen => (0, 255, 0),
            MyColor::BrightYellow => (255, 230, 0),
            MyColor::BrightBlue => (0, 0, 255),
            MyColor::BrightMagenta => (123, 0, 255),
            MyColor::BrightCyan => (0, 218, 247),
            MyColor::BrightWhite => (255, 255, 255),
            MyColor::RGB(r, g, b) => (r, g, b),
        };
        (r,g,b)
    }


        pub fn channel(&self, chan:Color_channel) -> u8 {
        let (r, g, b) = match *self {
            MyColor::Black => (0, 0, 0),
            MyColor::Red => (125, 0, 0),
            MyColor::Green => (0, 125, 0),
            MyColor::Yellow => (181, 165, 54),
            MyColor::Blue => (0, 0, 125),
            MyColor::Magenta => (76, 51, 102),
            MyColor::Cyan => (44, 167, 184),
            MyColor::White => (212, 210, 195),
            MyColor::BrightBlack => (33, 33, 33),
            MyColor::BrightRed => (255, 0, 0),
            MyColor::BrightGreen => (0, 255, 0),
            MyColor::BrightYellow => (255, 230, 0),
            MyColor::BrightBlue => (0, 0, 255),
            MyColor::BrightMagenta => (123, 0, 255),
            MyColor::BrightCyan => (0, 218, 247),
            MyColor::BrightWhite => (255, 255, 255),
            MyColor::RGB(r, g, b) => (r, g, b),
        };

        match chan {
            Color_channel::R => r,
            Color_channel::G => g,
            Color_channel::B => b,
        }
    }

    pub fn with_channel(&self,chan:Color_channel,v:u8) -> MyColor{
        let (r, g, b) = match *self {
            MyColor::Black => (0, 0, 0),
            MyColor::Red => (125, 0, 0),
            MyColor::Green => (0, 125, 0),
            MyColor::Yellow => (181, 165, 54),
            MyColor::Blue => (0, 0, 125),
            MyColor::Magenta => (76, 51, 102),
            MyColor::Cyan => (44, 167, 184),
            MyColor::White => (212, 210, 195),
            MyColor::BrightBlack => (33, 33, 33),
            MyColor::BrightRed => (255, 0, 0),
            MyColor::BrightGreen => (0, 255, 0),
            MyColor::BrightYellow => (255, 230, 0),
            MyColor::BrightBlue => (0, 0, 255),
            MyColor::BrightMagenta => (123, 0, 255),
            MyColor::BrightCyan => (0, 218, 247),
            MyColor::BrightWhite => (255, 255, 255),
            MyColor::RGB(r, g, b) => (r, g, b),
        };

        match chan {
            Color_channel::R => MyColor::RGB(v,g,b),
            Color_channel::G => MyColor::RGB(r,v,b),
            Color_channel::B => MyColor::RGB(r,g,v),
        }
    }

    pub fn to_color(&self) -> Color{
        let (r, g, b) = match *self {
            MyColor::Black => (0, 0, 0),
            MyColor::Red => (125, 0, 0),
            MyColor::Green => (0, 125, 0),
            MyColor::Yellow => (181, 165, 54),
            MyColor::Blue => (0, 0, 125),
            MyColor::Magenta => (76, 51, 102),
            MyColor::Cyan => (44, 167, 184),
            MyColor::White => (212, 210, 195),
            MyColor::BrightBlack => (33, 33, 33),
            MyColor::BrightRed => (255, 0, 0),
            MyColor::BrightGreen => (0, 255, 0),
            MyColor::BrightYellow => (255, 230, 0),
            MyColor::BrightBlue => (0, 0, 255),
            MyColor::BrightMagenta => (123, 0, 255),
            MyColor::BrightCyan => (0, 218, 247),
            MyColor::BrightWhite => (255, 255, 255),
            MyColor::RGB(r, g, b) => (r, g, b),
        };
        Color::Rgb(r, g, b)
    }


}


impl From<String> for MyColor{
    fn from(value: String) -> Self {
        let (r,g,b) = (&value[..2],&value[2..4],&value[4..6]);
        let (r,g,b) = (u8::from_str_radix(r, 16).unwrap(),u8::from_str_radix(g, 16).unwrap(),u8::from_str_radix(b, 16).unwrap());
        match (r,g,b) {
            (0,0,0) => MyColor::Black,
            (125,0,0) => MyColor::Red,
            (0,125,0) => MyColor::Green,
            (181,165,54) => MyColor::Yellow,
            (0,0,125) => MyColor::Blue,
            (76,51,102) => MyColor::Magenta,
            (44,167,184) => MyColor::Cyan,
            (212,210,195) => MyColor::White,
            (33,33,33) => MyColor::BrightBlack,
            (255,0,0) => MyColor::BrightRed,
            (0,255,0) => MyColor::BrightGreen,
            (255,230,0) => MyColor::BrightYellow,
            (0,0,255) => MyColor::BrightBlue,
            (123,0,255) => MyColor::BrightMagenta,
            (0,218,247) => MyColor::BrightCyan,
            (255,255,255) => MyColor::BrightWhite,
            _ => MyColor::RGB(r, g, b)
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

#[derive(Debug,PartialEq, Eq,Clone, Copy,Hash)]
pub enum Duration{
SECONDS(u8),
MIN(u8),
HOUR(u8),
DAY(u16),
WEEK(u8),
MONTH(u8),
YEAR(u8),
DECADE(u8),
CENTURY(u8),
Custom(u8,u8,u8,u16,u8,u8,u8),
Interval(u8,u8,u16,u8,u8,u8,u8,u8,u16,u8,u8,u8)
}

impl Duration{

pub fn from_raw_str(dur:&str) ->Self{
    if dur.contains("(") && dur.contains(")"){

        if let Some(start) = dur.find("("){

            if let Some(end) = dur.find(")"){
                if dur.to_lowercase().starts_with("custom"){
                let k = dur.to_string().split(",").map(|x| x.parse::<u16>().expect("Expected an unsigned integer")).collect::<Vec<u16>>();
                    if k.len() != 7{
                        std::process::exit(2);
                    }
                    return Duration::Custom(
                    u8::try_from(k[0]).unwrap(),
                    u8::try_from(k[1]).unwrap(),
                    u8::try_from(k[2]).unwrap(),
                    k[3],
                    u8::try_from(k[4]).unwrap(),
                    u8::try_from(k[5]).unwrap(),
                    u8::try_from(k[6]).unwrap(),
                    );
                }else{
                    let x = dur.to_string()[start+1..end].parse::<u16>().expect("Expected an unsigned integer");
                    return match &dur.to_string()[..start]{
                        "seconds" => Duration::SECONDS(u8::try_from(x).unwrap()),
                        "min" => Duration::MIN(u8::try_from(x).unwrap()),
                        "hour" => Duration::HOUR(u8::try_from(x).unwrap()),
                        "day" => Duration::DAY(x),
                        "week" => Duration::WEEK(u8::try_from(x).unwrap()),
                        "month" => Duration::MONTH(u8::try_from(x).unwrap()),
                        "year" => Duration::YEAR(u8::try_from(x).unwrap()),
                        "decade" => Duration::DECADE(u8::try_from(x).unwrap()),
                        "century" => Duration::CENTURY(u8::try_from(x).unwrap()),
                        _ => Duration::SECONDS(0)
                     };   
                    }
                }else{
                    return Duration::SECONDS(0);
                }
            } else{
                return Duration::SECONDS(0);
            }
                        
        }
    else{
        //  3/3/2025 10:12:56 AM -> 
        let (start,end) = dur.split_at(dur.find("->").unwrap());
        let (start,end) = (start.trim(),end.trim());
        let (d1,t1) = start.split_at(start.find(" ").unwrap());
        let (d2,t2) = end.split_at(end.find(" ").unwrap());

        let pt1 = t1.split(":").map(|x| x.parse::<u8>().expect("Expected Unsigned Integer")).collect::<Vec<u8>>();
        let pt2 = t2.split(":").map(|x| x.parse::<u8>().expect("Expected Unsigned Integer")).collect::<Vec<u8>>();
        let pd1 = d1.split("/").map(|x| x.parse::<u16>().expect("Expected Unsigned Integer")).collect::<Vec<u16>>();
        let pd2 = d2.split("/").map(|x| x.parse::<u16>().expect("Expected Unsigned Integer")).collect::<Vec<u16>>();
        if pt1.len() != 3 || pt2.len() != 3 || pd1.len() != 3 || pd2.len() != 3{
            eprintln!("Invalid date or time parse");
            std::process::exit(3);
        }
        return Duration::Interval(u8::try_from(pd1[0]).unwrap(),u8::try_from(pd1[1]).unwrap(),pd1[2], pt1[0], pt1[1], pt1[2], u8::try_from(pd2[0]).unwrap(),u8::try_from(pd2[1]).unwrap(),pd2[2], pt2[0], pt2[1], pt2[2]);        
    }
}

}


#[derive(Debug,PartialEq, Eq,Clone, Copy,Hash)]
pub enum Frequency{
Once,
SECONDS(u8),
MIN(u8),
HOUR(u8),
DAY(u16),
WEEK(u8),
MONTH(u8),
YEAR(u8),
DECADE(u8),
CENTURY(u8),
}

impl Frequency{

pub fn from_raw_str(freq:&str)  -> Self{
if let Some(start) = freq.find("("){
    if let Some(end) = freq.find(")"){
    if freq.starts_with("DAY"){
        return Frequency::DAY(freq.to_string()[start+1..end].parse::<u16>().expect("Expected Unsigned Integer"));
    }else{
        return match &freq.to_string()[..start]{
"SECONDS" => {Frequency::SECONDS(freq.to_string()[start+1..end].parse::<u8>().expect("Expected Unsigned Integer"))},
"MIN" => {Frequency::MIN(freq.to_string()[start+1..end].parse::<u8>().expect("Expected Unsigned Integer"))},
"HOUR" => {Frequency::HOUR(freq.to_string()[start+1..end].parse::<u8>().expect("Expected Unsigned Integer"))},
"WEEK" => {Frequency::WEEK(freq.to_string()[start+1..end].parse::<u8>().expect("Expected Unsigned Integer"))},
"MONTH" => {Frequency::MONTH(freq.to_string()[start+1..end].parse::<u8>().expect("Expected Unsigned Integer"))},
"YEAR" => {Frequency::YEAR(freq.to_string()[start+1..end].parse::<u8>().expect("Expected Unsigned Integer"))},
"DECADE" => {Frequency::DECADE(freq.to_string()[start+1..end].parse::<u8>().expect("Expected Unsigned Integer"))},
"CENTURY" => {Frequency::CENTURY(freq.to_string()[start+1..end].parse::<u8>().expect("Expected Unsigned Integer"))},
"Once" => {Frequency::Once},
_ => {Frequency::SECONDS(0)}};
    
    }
    }else{
    return Frequency::SECONDS(0);
    }

}else{
    return Frequency::SECONDS(0);
}


}

}




impl From<Frequency> for String{
fn from(value: Frequency) -> Self {
    match value{
Frequency::SECONDS(x) => format!("SECONDS({x})"),
Frequency::MIN(x) => format!("MIN({x})"),
Frequency::HOUR(x) => format!("HOUR({x})"),
Frequency::DAY(x) => format!("DAY({x})"),
Frequency::WEEK(x) => format!("WEEK({x})"),
Frequency::MONTH(x) => format!("MONTH({x})"),
Frequency::YEAR(x) => format!("YEAR({x})"),
Frequency::DECADE(x) => format!("DECADE({x})"),
Frequency::CENTURY(x) => format!("CENTURY({x})"),
Frequency::Once => format!("Once"),
    }
}

}





impl From<Duration> for String{
fn from(value: Duration) -> Self {
    match value{
Duration::SECONDS(x) => format!("SECONDS({x})"),
Duration::MIN(x) => format!("MIN({x})"),
Duration::HOUR(x) => format!("HOUR({x})"),
Duration::DAY(x) => format!("DAY({x})"),
Duration::WEEK(x) => format!("WEEK({x})"),
Duration::MONTH(x) => format!("MONTH{x}"),
Duration::YEAR(x) => format!("YEAR({x})"),
Duration::DECADE(x) => format!("DECADE({x})"),
Duration::CENTURY(x) => format!("CENTURY({x})"),
Duration::Custom(a,b ,c ,d ,e ,f ,g ) => format!("Custom({a},{b},{c},{d},{e},{f},{g})"),
Duration::Interval(a,b,c ,d ,e ,f ,g ,h ,i ,j ,k ,l ) => format!("{a}/{b}/{c} {d}:{e}:{f}->{g}/{h}/{i} {j}:{k}:{l}")
    }
}


}

#[derive(Debug,PartialEq, Eq,Clone, Copy)]
pub enum Txn_Type{
    DEBIT,
    CREDIT,
    BLOCKED
}

impl From<Txn_Type> for String{
    fn from(value: Txn_Type) -> Self {
        match value{
            Txn_Type::BLOCKED => format!("BLOCKED"),
            Txn_Type::DEBIT => format!("DEBIT"),
            Txn_Type::CREDIT => format!("CREDIT"),
        }
    }

}

impl From<String> for Txn_Type{
    fn from(value: String) -> Self {
        match &value[..]{
            "BLOCKEED" => Txn_Type::BLOCKED,
            "CREDIT" => Txn_Type::CREDIT,
            "DEBIT" => Txn_Type::DEBIT,
            _ => Txn_Type::BLOCKED
        }
    }

}



}