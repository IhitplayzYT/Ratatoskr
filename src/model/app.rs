pub mod App{
use std::{collections::HashSet, fs, io};


use crate::model::{journal::Journal::Journal_task, meta::Meta::{Tag,MyColor}, notes::Note::Note_task, todo::Todo::Todo_task};

use chrono::TimeZone;
use rust_decimal::Decimal;
use crate::Conversion::CONVERSION_RATES;
use serde::{Deserialize,Serialize};
pub struct Database {
    pub todos: HashSet<Todo_task>,
    pub notes: HashSet<Note_task>,
    pub journals: HashSet<Journal_task>,
    pub tags: HashSet<Tag>,
}

//pub struct App {
    //pub db: Database,
    //pub page: Page,
    //pub sidebar: SidebarState,
    //pub editor: EditorState,
    //pub popup: PopupState,
    //pub search: SearchState,
//}





#[derive(Debug,PartialEq, Eq,Clone, Copy)]
pub struct EditorState {
    pub mode: EditorMode,
    pub cursor_x: usize,
    pub cursor_y: usize,
    pub dirty: bool,
}

#[derive(Debug,PartialEq, Eq,Hash,Clone, Copy)]
pub enum EditorMode{
    Normal,
    Vim
}




#[derive(Serialize,Deserialize,Debug)]
pub struct Settings {
    pub theme: Theme,
    pub autosave: bool,
    pub autosave_freq: usize,
    pub vim_mode: bool,
    pub confirm_delete: bool,
    pub date_format: String,
    pub timezone: String,
    pub autocomplete: bool,
    pub currency: Currency
}

impl Settings{

fn new() -> Settings{
Self { theme:Default::default(),autocomplete:false, autosave: false,autosave_freq:usize::MAX, vim_mode: false, confirm_delete: true, date_format:"dd-mm-yyyy".to_string(),timezone:"Utc".to_string(),currency:Currency::INR}
}

fn save(&self) -> io::Result<()>{
fs::write("settings.json",serde_json::to_string_pretty(self)?)?;
Ok(())    
}

fn set(&mut self,other: Settings) {
self.theme = other.theme;
self.autosave = other.autosave;
self.vim_mode = other.vim_mode;
self.confirm_delete = other.confirm_delete;
self.date_format = other.date_format;
self.autosave_freq = other.autosave_freq;
self.timezone = other.timezone;
self.autocomplete = other.autocomplete;
self.currency = other.currency;
}

fn load(&mut self,path:Option<String>) -> bool{
self.set(serde_json::from_str::<Settings>(&fs::read_to_string(if let Some(x) = path {x} else{"settings.json".to_string()}).unwrap()).unwrap());
true
}

}







#[derive(Serialize,Deserialize,Debug)]
pub struct Theme {
    pub primary: MyColor,
    pub secondary: MyColor,
    pub accent: MyColor,
    pub success: MyColor,
    pub warning: MyColor,
    pub error: MyColor,
}

impl Default for Theme{
fn default() -> Self {
    Theme { primary: MyColor::Black, secondary: MyColor::White, accent: MyColor::BrightCyan, success: MyColor::BrightGreen, warning: MyColor::BrightYellow, error: MyColor::BrightRed }
}
}

impl Theme{

pub fn new(prim:Option<MyColor>,sec:Option<MyColor>,acc:Option<MyColor>,succ:Option<MyColor>,warning:Option<MyColor>,err:Option<MyColor>) -> Theme {
Self { primary: if let Some(x) = prim  {x} else{MyColor::Black},
       secondary: if let Some(x) = sec {x} else{MyColor::White},
       accent:  if let Some(x) = acc {x} else{MyColor::BrightCyan},
       success:  if let Some(x) = succ {x} else{MyColor::BrightGreen}, 
       warning:  if let Some(x) = warning {x} else{MyColor::BrightYellow},
       error:  if let Some(x) = err {x} else{MyColor::BrightRed} }
}

pub fn Save(&self) -> io::Result<()>{
    fs::write("theme.json",serde_json::to_string_pretty(self).unwrap())?;
    Ok(())
}
pub fn set(&mut self,new_t:Theme) {
self.primary = new_t.primary;
self.secondary = new_t.secondary;
self.accent = new_t.accent;
self.success = new_t.success;
self.warning = new_t.warning;
self.error = new_t.error;
}

pub fn Load(&mut self,conf_path: Option<String>){
    self.set(serde_json::from_str::<Theme>(&fs::read_to_string(if let Some(x) = conf_path{x} else {"theme.json".to_string()}).unwrap()).unwrap());
}


}


#[derive(Serialize,Deserialize,Debug,Hash,PartialEq, Eq,Clone, Copy)]
pub enum Currency {
    USD,
    EUR,
    GBP,
    INR,
    JPY,
    CNY,
    AUD,
    CAD,
    CHF,
    SEK,
    NOK,
    DKK,
    NZD,
    SGD,
    HKD,
    KRW,
    TWD,
    THB,
    MYR,
    IDR,
    PHP,
    VND,
    PKR,
    BDT,
    LKR,
    NPR,
    AED,
    SAR,
    QAR,
    KWD,
    BHD,
    OMR,
    ILS,
    TRY,
    RUB,
    UAH,
    PLN,
    CZK,
    HUF,
    RON,
    BGN,
    HRK,
    RSD,
    MXN,
    BRL,
    ARS,
    CLP,
    COP,
    PEN,
    ZAR,
    NGN,
    EGP,
    MAD,
    KES,
    ETB,
}
pub fn convert(
    amount: Decimal,
    to: Currency,
) -> Option<Decimal> {
    let map = CONVERSION_RATES.read().unwrap();
    let from_rate = map.get(&Currency::INR)?;
    let to_rate = map.get(&to)?;
    Some(amount / *from_rate * *to_rate)
}

impl Default for Currency{
fn default() -> Self {
    Self::INR
}

}

impl TryFrom<&str> for Currency {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "USD" => Ok(Self::USD),
            "EUR" => Ok(Self::EUR),
            "GBP" => Ok(Self::GBP),
            "INR" => Ok(Self::INR),
            "JPY" => Ok(Self::JPY),
            "CNY" => Ok(Self::CNY),
            "AUD" => Ok(Self::AUD),
            "CAD" => Ok(Self::CAD),
            "CHF" => Ok(Self::CHF),
            "SEK" => Ok(Self::SEK),
            "NOK" => Ok(Self::NOK),
            "DKK" => Ok(Self::DKK),
            "NZD" => Ok(Self::NZD),
            "SGD" => Ok(Self::SGD),
            "HKD" => Ok(Self::HKD),
            "KRW" => Ok(Self::KRW),
            "TWD" => Ok(Self::TWD),
            "THB" => Ok(Self::THB),
            "MYR" => Ok(Self::MYR),
            "IDR" => Ok(Self::IDR),
            "PHP" => Ok(Self::PHP),
            "VND" => Ok(Self::VND),
            "PKR" => Ok(Self::PKR),
            "BDT" => Ok(Self::BDT),
            "LKR" => Ok(Self::LKR),
            "NPR" => Ok(Self::NPR),
            "AED" => Ok(Self::AED),
            "SAR" => Ok(Self::SAR),
            "QAR" => Ok(Self::QAR),
            "KWD" => Ok(Self::KWD),
            "BHD" => Ok(Self::BHD),
            "OMR" => Ok(Self::OMR),
            "ILS" => Ok(Self::ILS),
            "TRY" => Ok(Self::TRY),
            "RUB" => Ok(Self::RUB),
            "UAH" => Ok(Self::UAH),
            "PLN" => Ok(Self::PLN),
            "CZK" => Ok(Self::CZK),
            "HUF" => Ok(Self::HUF),
            "RON" => Ok(Self::RON),
            "BGN" => Ok(Self::BGN),
            "HRK" => Ok(Self::HRK),
            "RSD" => Ok(Self::RSD),
            "MXN" => Ok(Self::MXN),
            "BRL" => Ok(Self::BRL),
            "ARS" => Ok(Self::ARS),
            "CLP" => Ok(Self::CLP),
            "COP" => Ok(Self::COP),
            "PEN" => Ok(Self::PEN),
            "ZAR" => Ok(Self::ZAR),
            "NGN" => Ok(Self::NGN),
            "EGP" => Ok(Self::EGP),
            "MAD" => Ok(Self::MAD),
            "KES" => Ok(Self::KES),
            "ETB" => Ok(Self::ETB),
            _ => Err(()),
        }
    }
}

}