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

    #[derive(Debug,PartialEq, Eq,Hash)]
pub enum Mood {
    Happy,
    Neutral,
    Sad,
    Angry,
    Excited,
    Tired,
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