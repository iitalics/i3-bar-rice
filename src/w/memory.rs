use std::iter;
use regex;
use render::Elem;
use fillbar::FillBar;
use super::Widget;
use super::read_program;
use super::super::{Result, Error};

pub struct MemoryW {
    re: regex::Regex,
    fillbar: FillBar,
}

impl MemoryW {

    pub fn new() -> Self {
        let re = regex::Regex::new(r"Mem:\s+(\d+)\s+(\d+)\s+(\d+)").unwrap();

        let fillbar = FillBar::single(12, "#20ba00");

        MemoryW { re, fillbar }
    }

    fn parse_free(&self) -> Result<(i32, i32)> {
        let out = read_program("/bin/free")?;

        let caps = self.re.captures(&out)
                          .ok_or(Error::BadCmdFormat("free doesn't match regex"))?;

        let kb_use = caps[2].parse::<i32>().map_err(|_| Error::BadCmdFormat("invalid int"))?;
        let kb_tot = caps[1].parse::<i32>().map_err(|_| Error::BadCmdFormat("invalid int"))?;

        Ok(( kb_use / 1024,
             kb_tot / 1024 ))
    }
}

impl Widget for MemoryW {
    fn cooldown(&self) -> i32 { 4 }

    fn blit(&self) -> Result<Vec<Elem>> {
        let (mb_use, mb_tot) = self.parse_free()?;

        Ok(iter::once(Elem::plain("mem: ["))
           .chain(self.fillbar.blit('\\', mb_use, mb_tot))
           .chain(vec![Elem::plain("] "),
                       Elem::new(format!("{}mb", mb_use), "#3d3d53")])
           .collect())
    }
}
