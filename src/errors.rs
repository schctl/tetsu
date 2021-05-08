use nbt::Error as nbt_error;
use serde_json::Error as serde_error;
use std::io;
use std::string;

use quick_error::quick_error;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io {
            from(io::Error)
        }
        FromUtf8 {
            from(string::FromUtf8Error)
        }
        Serde {
            from(serde_error)
        }
        Nbt {
            from(nbt_error)
        }
    }
}
