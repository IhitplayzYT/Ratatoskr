pub mod Pomodero{
    use crate::model::{app::App::PomoFocus, meta::Meta::MyColor};


    pub struct Pomodero{
        pub duration:usize,
        pub duration_left:usize,
        pub fmt: String,
        pub color:MyColor,
        pub running: bool,
        pub paused: bool,
        pub focus: PomoFocus,
    }

    impl Default for Pomodero{
        fn default() -> Self {
            Self { duration_left:600,duration: 600, fmt: "{hh}:{mm}:{ss}".to_string(), color: MyColor::BrightCyan,running:false,paused:false,focus:PomoFocus::Hour}
        }
    }

    impl Pomodero{

    pub fn format_time_left(&self) -> String {
        let hours = self.duration_left / 3600;
        let minutes = (self.duration_left % 3600) / 60;
        let seconds = self.duration_left % 60;

        self.fmt
            .replace("{hh}", &format!("{:02}", hours))
            .replace("{h}", &hours.to_string())
            .replace("{mm}", &format!("{:02}", minutes))
            .replace("{m}", &minutes.to_string())
            .replace("{ss}", &format!("{:02}", seconds))
            .replace("{s}", &seconds.to_string())
        }           

    pub fn new(duration:usize,fmt:Option<String>,color:Option<MyColor>) ->Self{
        Self { duration,duration_left:duration, fmt:fmt.unwrap_or("{hh}:{mm}:{ss}".to_string()) , color: MyColor::BrightCyan,running:false,paused:false,focus:PomoFocus::Hour}
    }

    }

}