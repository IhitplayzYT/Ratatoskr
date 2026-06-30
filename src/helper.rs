pub mod Helper {

    const DBG_STR: &str = "";
    const OK: i32 = 0;
    const ERR: i32 = 1;

    #[derive(Debug)]
    pub struct CLI {
        pub debug: bool,
        pub srcdir: Option<String>,
        pub ifile: Option<String>,
        pub port:u16,
        pub url: Option<String>,
        pub user: String,
        pub password: Option<String>
    }

    impl CLI {
        pub fn new() -> CLI {
            Self {
                debug: false,
                srcdir: None,
                ifile: None,
                port:8080,
                url:None,
                user:"root".to_string(),
                password: None
            }
        }

        pub fn Parse_Args(&mut self) {
            let clargs = std::env::args().collect::<Vec<String>>();
            self.user = match std::env::var("DB_USERNAME") {
               Ok(u) => u,
               _ => "root".to_string()
            };
            self.password = match std::env::var("DB_PASSWORD") {
               Ok(u) => {if u.is_empty() {None} else {Some(u)}},
               _ => None
            };
            if clargs.is_empty() {
                return;
            }
            for i in &clargs {
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
                }
                 else {
                    Help();
                }
            }
        }
    }

    pub fn Help() {
        println!("{DBG_STR}");
        std::process::exit(OK);
    }
}
