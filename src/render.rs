use std;
use serde_json;

#[derive(Clone, Debug)]
pub struct Elem {
    pub text: String,
    pub color: String,
}

#[derive(Serialize)]
struct JSONVersion {
    version: i32,
}

#[derive(Serialize)]
struct JSONElem {
    full_text: String,
    color: String,
    separator: bool,
    separator_block_width: i32,
}


impl Elem {

    pub fn new<T,C>(text: T, color: C) -> Self
        where T: Into<String>,
              C: Into<String> {
        Elem { text: text.into(), color: color.into() }
    }

    pub fn plain<T>(text: T) -> Self
        where T: Into<String> {
        Elem { text: text.into(), color: "#ffffff".into() }
    }

    fn into_json(self) -> JSONElem {
        JSONElem {
            full_text: self.text,
            color: self.color,
            separator: false,
            separator_block_width: 0,
        }
    }
}


pub fn version_json_string() -> String {
    serde_json::to_string(&JSONVersion { version: 1 }).unwrap()
}

pub fn render_stream<I,J>(dt_ms: u64, blits: I) -> std::io::Result<()>
    where I: IntoIterator<Item=J>,
          J: IntoIterator<Item=Elem> {

    use std::io::{self, BufWriter, Write};
    use std::time::Duration;

    // lock handle to stdout
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    // write version string
    write!(out, "{}\n[", version_json_string())?;
    out.flush()?;

    // iterate and render from blit stream
    for blit in blits {

        serde_json::to_writer(
            &mut out,
            &blit.into_iter()
                 .map(|e| e.into_json())
                 .collect::<Vec<_>>())?;

        out.write(",".as_bytes())?;
        out.flush()?;

        std::thread::sleep(Duration::from_millis(dt_ms));
    }

    Ok(())
}
