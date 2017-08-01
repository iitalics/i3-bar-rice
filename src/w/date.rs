use render::Elem;
use super::super::Result;
use super::Widget;

static MONTHS: [&'static str; 12] =
    [ "Jan", "Feb", "Mar", "Apr", "May", "Jun",
       "Jul", "Aug", "Sep", "Oct", "Nov", "Dec" ];

pub struct DateTimeW;

impl DateTimeW {
    pub fn new() -> Self { DateTimeW }
}

impl Widget for DateTimeW {
    fn blit(&self) -> Result<Vec<Elem>> {
        use chrono::prelude::*;
        let now = Local::now();

        let time_str =
            format!("{} {}    {:02}:{:02}:{:02}",
                    MONTHS[now.month0() as usize],
                    now.day(),
                    now.hour(),
                    now.minute(),
                    now.second());

        Ok(vec![Elem::plain(time_str)])
    }
}
