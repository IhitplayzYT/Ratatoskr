#[allow(dead_code,non_camel_case_types,non_snake_case)]

pub mod App{
use std::{collections::HashSet, fs, hash::Hash, io, time::Instant};


use crate::{db::Database::Database, model::{calendar::Calendar::Calendar_task, finance::Finance::Ledger, journal::Journal::Journal_task, meta::Meta::{MyColor, Tag}, notes::Note::Note_task, pomodero::Pomodero, todo::Todo::Todo_task}};

use chrono::{Utc};
use ratatui::layout::Rect;
use rust_decimal::Decimal;
use crate::Conversion::CONVERSION_RATES;
use serde::{Deserialize,Serialize};
use ropey::Rope;
pub struct Feature_set {
    pub todos: HashSet<Todo_task>,
    pub notes: HashSet<Note_task>,
    pub journals: HashSet<Journal_task>,
    pub finance: Ledger,
    pub calendars: HashSet<Calendar_task>,
    pub tags: HashSet<Tag>,
    pub pomodero:Pomodero::Pomodero,
}

// TODO: LOAD THESE FEATURES FROM DB

impl Default for Feature_set{
    fn default() -> Self {
        Self { todos: HashSet::new(), notes: HashSet::new(), journals: HashSet::new(), finance: Ledger::new(), calendars: HashSet::new(), tags: HashSet::new() ,pomodero:Pomodero::Pomodero::default()}
    }

}



pub struct App {
    pub features: Feature_set,
    pub db: Database,
    pub settings:Settings,
    pub page:Page,
    pub editor:EditorState,
    pub last_editor_section: Rect,
    pub is_quit: bool,
    pub pomo_last_second: std::time::Instant,
}

impl App {
    pub fn new(url:String) -> Self {
        Self {
            features: Feature_set::default(),
            db: Database::new(&url).unwrap(),
            settings: Settings::default(),
            page: Page::Home,
            editor: EditorState::default(),
            last_editor_section:Rect::default(),
            is_quit: false,
            pomo_last_second:Instant::now()
        }
    }
}



pub enum Page{
    Home,
    Journal,
    Note,
    Todo,
    Calendar,
    Finance,
    Pomodoro,
    Settings
}

impl Page{
    pub const ALL:[Page;8] = [
        Page::Home,
        Page::Journal,
        Page::Note,
        Page::Todo,
        Page::Calendar,
        Page::Pomodoro,
        Page::Finance,
        Page::Settings,
    ];

    pub fn title(&self) -> &'static str{
        match self {
            Page::Home => "Home",
            Page::Journal => "Journal",
            Page::Note => "Note",
            Page::Todo => "Todo",
            Page::Calendar => "Calendar",
            Page::Pomodoro => "Pomodoro",
            Page::Finance => "Finance",
            Page::Settings => "Settings",
        }
    }

    pub fn idx(&self) -> usize {
        match self {
            Page::Home => 0,
            Page::Journal => 1,
            Page::Note => 2,
            Page::Todo => 3,
            Page::Calendar => 4,
            Page::Pomodoro => 5,
            Page::Finance => 6,
            Page::Settings => 7,
        }
    }

    pub fn from_idx(i: usize) -> Page {
        match i % 8 {
            0 => Page::Home,
            1 => Page::Journal,
            2 => Page::Note,
            3 => Page::Todo,
            4 => Page::Calendar,
            5 => Page::Pomodoro,
            6 => Page::Finance,
            _ => Page::Settings,
        }
    }


}


#[derive(Debug,PartialEq, Eq,Clone)]
pub struct EditorState {
    pub mode: EditorMode,
    pub cursor_x: usize,
    pub cursor_y: usize,
    pub dirty: bool,
    pub buffer: Rope, // TODO: swap per active task/page instead of one shared demo buffer
    pub vim: Vimstate,
}

#[derive(Debug,PartialEq, Eq,Hash,Clone, Copy)]
pub enum EditorMode{
    Normal,
    Vim
}

impl Default for EditorState{
    fn default() -> Self {
        Self { mode: EditorMode::Normal, cursor_x: 0, cursor_y: 0, dirty: false,buffer:Rope::from_str("Start Typing\nRope"),vim:Vimstate::default()}
    }

}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Vim_mode {
    Normal,
    Insert,
}

/// for mutli key vim command
#[derive(Debug, PartialEq, Eq, Clone, Copy,)]
pub enum Pending {
    None,
    G,  // 'g'
    Y,  // 'y' (yy) or 'i' (yiw)
    YI, // ''w'
    D,  // 'd' (dd) or 'i' (diw)
    DI, // 'w'
    R,  // replacement char
}
 
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum PomoFocus {
    Hour,
    Minute,
    Second,
    StartPause,
    Reset,
    Cancel,
}


impl PomoFocus{
    const ALL:[PomoFocus;6] = [
    PomoFocus::Hour,
    PomoFocus::Minute,
    PomoFocus::Second,
    PomoFocus::StartPause,
    PomoFocus::Reset,
    PomoFocus::Cancel,
    ];

    pub fn title(&self) -> String{
        match self{
    PomoFocus::Hour => "Hour",
    PomoFocus::Minute => "Minute",
    PomoFocus::Second => "Second",
    PomoFocus::StartPause => "Start/Pause",
    PomoFocus::Reset => "Reset",
    PomoFocus::Cancel => "Cancel",   
        }.to_string()
    }

    pub fn idx(&self) -> usize{
        match self{
    PomoFocus::Hour => 0,
    PomoFocus::Minute => 1,
    PomoFocus::Second => 2,
    PomoFocus::StartPause => 3,
    PomoFocus::Reset => 4,
    PomoFocus::Cancel => 5,

        }
    }

    pub fn from_idx(&self,idx:usize) -> Self{
        match idx{
        0 => PomoFocus::Hour,
        1 => PomoFocus::Minute,
        2 => PomoFocus::Second,
        3 => PomoFocus::StartPause,
        4 => PomoFocus::Reset,
        5 => PomoFocus::Cancel,
        _ => PomoFocus::Cancel
        }
    }

    pub fn next(self) -> PomoFocus { let i = Self::ALL.iter().position(|c| *c == self).unwrap(); Self::ALL[(i + 1) % Self::ALL.len()] }
    pub fn prev(self) -> PomoFocus { let i = Self::ALL.iter().position(|c| *c == self).unwrap(); Self::ALL[(i + Self::ALL.len() - 1) % Self::ALL.len()] }

}




#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Vimstate {
    pub submode: Vim_mode,
    pub pending: Pending,
    pub register: String,
    pub register_line: bool,
}
 
impl Default for Vimstate {
    fn default() -> Self {
        Self {
            submode: Vim_mode::Normal,
            pending: Pending::None,
            register: String::new(),
            register_line: false,
        }
    }
}




#[derive(Debug, Clone, Copy,Serialize,Deserialize,PartialEq, Eq)]
pub enum Color_channel {
    R,
    G,
    B,
}

impl Color_channel{
    pub fn next(self) -> Color_channel {
        match self {
            Color_channel::R => Color_channel::G,
            Color_channel::G => Color_channel::B,
            Color_channel::B => Color_channel::R,
        }
    }
    pub fn prev(self) -> Color_channel {
        match self {
            Color_channel::R => Color_channel::B,
            Color_channel::G => Color_channel::R,
            Color_channel::B => Color_channel::G,
        }
    }


}

/// Live/interactive RGB color picker state.
#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct ColorPickerState {
    pub selected: Color_channel,
    pub component: Theme_comp,
    pub buffers: [String;3],
}
 
impl ColorPickerState {
    pub fn new(theme: &Theme) -> Self {
        let mut s = Self {
            component: Theme_comp::Primary,
            selected: Color_channel::R,
            buffers: [String::new(), String::new(), String::new()],
        };
        s.reload_from_theme(theme);
        s
    }
 
    /// Refill the text buffers from the theme (call after switching component).
    pub fn reload_from_theme(&mut self, theme: &Theme) {
        let c = self.component.get(theme);
        self.buffers = [
            c.channel(Color_channel::R).to_string(),
            c.channel(Color_channel::G).to_string(),
            c.channel(Color_channel::B).to_string(),
        ];
    }
 
    pub fn buffer_mut(&mut self, ch: Color_channel) -> &mut String {
        match ch {
            Color_channel::R => &mut self.buffers[0],
            Color_channel::G => &mut self.buffers[1],
            Color_channel::B => &mut self.buffers[2],
        }
    }
 
    /// Parse current focused buffer, clamp 0-255, write straight into theme
    /// live as you type.
    pub fn commit_focused_channel(&mut self, theme: &mut Theme) {
        let raw = self.buffer_mut(self.selected).clone();
        let value: u8 = raw.parse::<u32>().unwrap_or(0).min(255) as u8;
        let updated = self.component.get(theme).with_channel(self.selected,value);
        self.component.set(theme, updated);
    }
}




#[derive(Serialize,Deserialize,Debug)]

pub struct Settings {
    pub color_picker:ColorPickerState,
    pub theme: Theme,
    pub autosave: bool,
    pub autosave_freq: usize,
    pub vim_mode: bool,
    pub confirm_delete: bool,
    pub date_format: String,
    pub timezone: String,
    pub autocomplete: bool,
    pub currency: Currency,
    pub settings_ui: SettingsUiState,
}

impl Default for Settings{
    fn default() -> Self {
        let theme = Theme::default();
        Self { color_picker: ColorPickerState::new(&theme), theme, autosave: false, autosave_freq: 0, vim_mode: false, confirm_delete: true, date_format: "dd-mm-yyyy".to_string(), timezone: "Utc".to_string(), autocomplete: false, currency: Currency::INR,settings_ui:SettingsUiState { autosave_freq_input: "0 sec".to_string(), autosave_freq_valid: false, timezone_input: "UTC".to_string(),..Default::default()}}
    }

}


impl Settings{

pub fn new() -> Settings{
let theme = Theme::default();
Self { theme,autocomplete:false, autosave: false,autosave_freq:0, vim_mode: false, confirm_delete: true, date_format:"dd-mm-yyyy".to_string(),timezone:"Utc".to_string(),currency:Currency::INR,color_picker:ColorPickerState::new(&theme),settings_ui:SettingsUiState {autosave_freq_input: "0 sec".to_string(), autosave_freq_valid: false, timezone_input: "Utc".to_string(),..Default::default()}}
}

pub fn save(&self) -> io::Result<()>{
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
self.color_picker = other.color_picker;
}

pub fn load(&mut self,path:String) -> bool{
self.set(serde_json::from_str::<Settings>(&fs::read_to_string(path).unwrap()).unwrap());
true
}

}


#[derive(Debug, Clone, Copy, PartialEq, Eq,Serialize,Deserialize)]
pub enum SettingsFocus { ColorPicker, AutosaveEnabled, AutosaveFreq,AutocompleteEnabled,VimmodeEnabled,ConfirmdeleteEnabled,Datefmt, Currency, Timezone}

impl Default for SettingsFocus{
    fn default() -> Self {
        Self::ColorPicker
    }

}


#[derive(Debug, Clone, Default,Deserialize,Serialize)]
pub struct SettingsUiState {
    pub focus: SettingsFocus, // impl Default -> ColorPicker
    pub autosave_freq_input: String, // raw typed text, e.g. "2 hr 30 min"
    pub autosave_freq_valid: bool,
    pub autocomplete:bool,
    pub vim_mod: bool,
    pub confirm_delete:bool,
    pub date_fmt: String,
    pub date_valid:bool,
    pub timezone_input: String,
}

pub fn parse_fmt_date(fmtstr:&str) -> Result<String,std::fmt::Error>{
    use std::fmt::Write;
    let mut out = String::new();
    write!(out, "{}", Utc::now().format(fmtstr))?;
    Ok(out)
}


/// Parses "5 min", "60 sec", "2 hr 30 min" into seconds.
pub fn parse_autosave_duration(input: &str) -> Option<usize> {
    let mut total = 0usize;
    let mut chars = input.trim().chars().peekable();
    let mut saw_any = false;
    while chars.peek().is_some() {
        while chars.peek().map_or(false, |c| c.is_whitespace()) { chars.next(); }
        let mut num = String::new();
        while chars.peek().map_or(false, |c| c.is_ascii_digit()) { num.push(chars.next().unwrap()); }
        if num.is_empty() { return None; }
        while chars.peek().map_or(false, |c| c.is_whitespace()) { chars.next(); }
        let mut unit = String::new();
        while chars.peek().map_or(false, |c| c.is_alphabetic()) { unit.push(chars.next().unwrap()); }
        let n: usize = num.parse().ok()?;
        let mult = match unit.to_lowercase().as_str() {
            "s" | "sec" | "secs" | "second" | "seconds" => 1,
            "m" | "min" | "mins" | "minute" | "minutes" => 60,
            "h" | "hr" | "hrs" | "hour" | "hours" => 3600,
            "d" | "day" | "days" => 86400,
            _ => return None,
        };
        total += n * mult;
        saw_any = true;
        while chars.peek().map_or(false, |c| c.is_whitespace()) { chars.next(); }
    }
    saw_any.then_some(total)
}



#[derive(Serialize,Deserialize,Debug,Clone,Copy)]
pub struct Theme {
    pub primary: MyColor,
    pub secondary: MyColor,
    pub accent: MyColor,
    pub success: MyColor,
    pub warning: MyColor,
    pub error: MyColor,
    pub fontfg:MyColor,
    pub fontbg:MyColor,
}

impl Default for Theme{
fn default() -> Self {
    Theme { primary: MyColor::RGB(200, 200, 200), secondary: MyColor::White, accent: MyColor::BrightCyan, success: MyColor::BrightGreen, warning: MyColor::BrightYellow, error: MyColor::BrightRed,fontbg:MyColor::Black,fontfg:MyColor::BrightWhite}
}
}

impl Theme{

pub fn new(prim:Option<MyColor>,sec:Option<MyColor>,acc:Option<MyColor>,succ:Option<MyColor>,warning:Option<MyColor>,err:Option<MyColor>,fg:Option<MyColor>,bg:Option<MyColor>) -> Theme {
Self { primary: if let Some(x) = prim  {x} else{MyColor::Black},
       secondary: if let Some(x) = sec {x} else{MyColor::White},
       accent:  if let Some(x) = acc {x} else{MyColor::BrightCyan},
       success:  if let Some(x) = succ {x} else{MyColor::BrightGreen}, 
       warning:  if let Some(x) = warning {x} else{MyColor::BrightYellow},
       error:  if let Some(x) = err {x} else{MyColor::BrightRed},
       fontfg: fg.unwrap_or(MyColor::BrightWhite),
       fontbg: bg.unwrap_or(MyColor::Black),
    }
       
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
self.fontfg = new_t.fontfg;
self.fontbg = new_t.fontbg;
}

pub fn Load(&mut self,conf_path: Option<String>){
    self.set(serde_json::from_str::<Theme>(&fs::read_to_string(if let Some(x) = conf_path{x} else {"theme.json".to_string()}).unwrap()).unwrap());
}


}

#[derive(Debug, Clone, Copy, PartialEq, Eq,Serialize,Deserialize)]
pub enum Theme_comp {
    Primary,
    Secondary,
    Accent,
    Success,
    Warning,
    Error,
    FontFG,
    FontBG
}
 
impl Theme_comp {
    pub const ALL: [Theme_comp; 8] = [
        Theme_comp::Primary,
        Theme_comp::Secondary,
        Theme_comp::Accent,
        Theme_comp::Success,
        Theme_comp::Warning,
        Theme_comp::Error,
        Theme_comp::FontFG,
        Theme_comp::FontBG,
    ];
 
    pub fn title(self) -> &'static str {
        match self {
            Theme_comp::Primary => "Primary",
            Theme_comp::Secondary => "Secondary",
            Theme_comp::Accent => "Accent",
            Theme_comp::Success => "Success",
            Theme_comp::Warning => "Warning",
            Theme_comp::Error => "Error",
            Theme_comp::FontFG => "FontFG",
            Theme_comp::FontBG => "FontBG",
        }
    }
 
    pub fn get(self, theme: &Theme) -> MyColor {
        match self {
            Theme_comp::Primary => theme.primary,
            Theme_comp::Secondary => theme.secondary,
            Theme_comp::Accent => theme.accent,
            Theme_comp::Success => theme.success,
            Theme_comp::Warning => theme.warning,
            Theme_comp::Error => theme.error,
            Theme_comp::FontFG => theme.fontfg,
            Theme_comp::FontBG => theme.fontbg,
        }
    }
 
    pub fn set(self, theme: &mut Theme, c: MyColor) {
        let slot = match self {
            Theme_comp::Primary => &mut theme.primary,
            Theme_comp::Secondary => &mut theme.secondary,
            Theme_comp::Accent => &mut theme.accent,
            Theme_comp::Success => &mut theme.success,
            Theme_comp::Warning => &mut theme.warning,
            Theme_comp::Error => &mut theme.error,
            Theme_comp::FontBG => &mut theme.fontbg,
            Theme_comp::FontFG => &mut theme.fontfg,
        };
        *slot = c;
    }
 
    pub fn next(self) -> Theme_comp {
        let i = Theme_comp::ALL.iter().position(|c| *c == self).unwrap();
        Theme_comp::ALL[(i + 1) % Theme_comp::ALL.len()]
    }
    pub fn prev(self) -> Theme_comp {
        let i = Theme_comp::ALL.iter().position(|c| *c == self).unwrap();
        Theme_comp::ALL[(i + Theme_comp::ALL.len() - 1) % Theme_comp::ALL.len()]
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


impl Currency {
    const ALL: [Currency; 55] = [    Currency::USD,
    Currency::EUR,
    Currency::GBP,
    Currency::INR,
    Currency::JPY,
    Currency::CNY,
    Currency::AUD,
    Currency::CAD,
    Currency::CHF,
    Currency::SEK,
    Currency::NOK,
    Currency::DKK,
    Currency::NZD,
    Currency::SGD,
    Currency::HKD,
    Currency::KRW,
    Currency::TWD,
    Currency::THB,
    Currency::MYR,
    Currency::IDR,
    Currency::PHP,
    Currency::VND,
    Currency::PKR,
    Currency::BDT,
    Currency::LKR,
    Currency::NPR,
    Currency::AED,
    Currency::SAR,
    Currency::QAR,
    Currency::KWD,
    Currency::BHD,
    Currency::OMR,
    Currency::ILS,
    Currency::TRY,
    Currency::RUB,
    Currency::UAH,
    Currency::PLN,
    Currency::CZK,
    Currency::HUF,
    Currency::RON,
    Currency::BGN,
    Currency::HRK,
    Currency::RSD,
    Currency::MXN,
    Currency::BRL,
    Currency::ARS,
    Currency::CLP,
    Currency::COP,
    Currency::PEN,
    Currency::ZAR,
    Currency::NGN,
    Currency::EGP,
    Currency::MAD,
    Currency::KES,
    Currency::ETB];
    pub fn title(self) -> &'static str {
        match self{
            Currency::USD => "USD",
            Currency::EUR => "EUR",
            Currency::GBP => "GBP",
            Currency::INR => "INR",
            Currency::JPY => "JPY",
            Currency::CNY => "CNY",
            Currency::AUD => "AUD",
            Currency::CAD => "CAD",
            Currency::CHF => "CHF",
            Currency::SEK => "SEK",
            Currency::NOK => "NOK",
            Currency::DKK => "DKK",
            Currency::NZD => "NZD",
            Currency::SGD => "SGD",
            Currency::HKD => "HKD",
            Currency::KRW => "KRW",
            Currency::TWD => "TWD",
            Currency::THB => "THB",
            Currency::MYR => "MYR",
            Currency::IDR => "IDR",
            Currency::PHP => "PHP",
            Currency::VND => "VND",
            Currency::PKR => "PKR",
            Currency::BDT => "BDT",
            Currency::LKR => "LKR",
            Currency::NPR => "NPR",
            Currency::AED => "AED",
            Currency::SAR => "SAR",
            Currency::QAR => "QAR",
            Currency::KWD => "KWD",
            Currency::BHD => "BHD",
            Currency::OMR => "OMR",
            Currency::ILS => "ILS",
            Currency::TRY => "TRY",
            Currency::RUB => "RUB",
            Currency::UAH => "UAH",
            Currency::PLN => "PLN",
            Currency::CZK => "CZK",
            Currency::HUF => "HUF",
            Currency::RON => "RON",
            Currency::BGN => "BGN",
            Currency::HRK => "HRK",
            Currency::RSD => "RSD",
            Currency::MXN => "MXN",
            Currency::BRL => "BRL",
            Currency::ARS => "ARS",
            Currency::CLP => "CLP",
            Currency::COP => "COP",
            Currency::PEN => "PEN",
            Currency::ZAR => "ZAR",
            Currency::NGN => "NGN",
            Currency::EGP => "EGP",
            Currency::MAD => "MAD",
            Currency::KES => "KES",
            Currency::ETB => "ETB",
        }
    }
    pub fn next(self) -> Currency { let i = Self::ALL.iter().position(|c| *c == self).unwrap(); Self::ALL[(i + 1) % Self::ALL.len()] }
    pub fn prev(self) -> Currency { let i = Self::ALL.iter().position(|c| *c == self).unwrap(); Self::ALL[(i + Self::ALL.len() - 1) % Self::ALL.len()] }
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