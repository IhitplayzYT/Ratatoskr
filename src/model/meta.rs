pub mod Meta{
    use serde::{Deserialize, Serialize};
use uuid::Uuid;
    use colored::Color;

    #[derive(Debug,PartialEq, Eq,Hash)]
    pub enum Priority {
    Low,
    Default,
    Medium,
    High,
    Critical,
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


}