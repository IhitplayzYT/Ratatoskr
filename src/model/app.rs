pub mod App{
use std::{collections::HashSet, fs};


use crate::model::{journal::Journal::Journal_task, meta::Meta::{Tag,MyColor}, notes::Note::Note_task, todo::Todo::Todo_task};

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

//pub struct SearchState {
    //pub query: String,
    //pub results: Vec<SearchResult>,
    //pub cursor: usize,
//}

//pub struct EditorState {
    //pub mode: EditorMode,
    //pub cursor_x: usize,
    //pub cursor_y: usize,
    //pub dirty: bool,
//}

#[derive(Serialize,Deserialize,Debug)]
pub struct Settings {
    pub theme: Theme,
    pub autosave: bool,
    pub vim_mode: bool,
    pub confirm_delete: bool,
    pub date_format: String,
}

impl Settings{

fn new() -> Settings{
Self { theme:Default::default(), autosave: false, vim_mode: false, confirm_delete: true, date_format:"dd-mm-yyyy".to_string()}
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

pub fn Save(&self) {
    fs::write("theme.json",serde_json::to_string_pretty(self).unwrap());
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


}