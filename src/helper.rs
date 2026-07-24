#[allow(dead_code,non_camel_case_types,non_snake_case)]

pub mod Helper {

    const DBG_STR: &str = "Usage";
    const OK: i32 = 0;
    const ERR: i32 = 1;

    #[derive(Debug,Clone)]
    pub struct CLI {
        pub debug: bool,
        pub srcdir: Option<String>,
        pub ifile: Option<String>,
        pub port:u16,
        pub url: Option<String>,
        pub user: String,
        pub password: Option<String>,
        pub database:String,
        pub settings: Option<String>,
        pub clear: bool,
    }

    impl CLI {
        pub fn new() -> CLI {
            Self {
                debug: false,
                srcdir: None,
                ifile: None,
                port:3306,
                url:None,
                user:"root".to_string(),
                password: None,
                settings:None,
                database:"mydb".to_string(),
                clear: false,
            }
        }

        pub fn Parse_Args(&mut self) {
            let clargs = std::env::args().collect::<Vec<String>>();
            self.user = match std::env::var("DB_USER") {
               Ok(u) => u,
               _ => "root".to_string()
            };
            self.password = match std::env::var("DB_PASSWD") {
               Ok(u) => {if u.is_empty() {None} else {Some(u)}},
               _ => None
            };
            if clargs.is_empty() {
                return;
            }
            for i in clargs.iter().skip(1).collect::<Vec<&String>>() {
                
                if i == "-h" || i == "--help" || i == "-H" || i == "--Help" {
                    Help();
                } else if i == "-d" || i == "--debug" || i == "-D" || i == "--Debug" {
                    self.debug = true
                } else if i.starts_with("--src=") || i.starts_with("--SRC=") {
                    let idx = i.find("=").unwrap();
                    self.srcdir = Some(i[idx + 1..].to_string());
                } else if i.starts_with("--file=") || i.starts_with("--FILE=") {
                    let idx = i.find("=").unwrap();
                    self.ifile = Some(i[idx + 1..].to_string());
                } else if i.starts_with("--port=") || i.starts_with("-p=") {
                    let idx = i.find("=").unwrap();
                    self.port = i[idx + 1..].parse::<u16>().expect("Port is a unsigned 16 bit integer(0 - 65536)");
                } else if i.starts_with("--url=") || i.starts_with("-u=") {
                    let idx = i.find("=").unwrap();
                    self.url = Some(i[idx + 1..].to_string());
                } else if i.starts_with("--settings=") || i.starts_with("-s="){
                    let idx = i.find("=").unwrap();
                    self.settings = Some(i[idx + 1..].to_string());                   
                } else if i.starts_with("--database=") || i.starts_with("-db="){
                    let idx = i.find("=").unwrap();
                    self.database = i[idx + 1..].to_string();
                } else if i == "--clear" || i == "-c"{
                    self.clear = true;
                }
                 else {
                    Help();
                }
            }
            println!("{:?}",self);
        }
    }

    pub fn Help() {
        println!("{DBG_STR}");
        std::process::exit(OK);
    }
}
