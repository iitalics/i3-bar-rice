use regex;
use std::iter;
use regex::Regex;
use render::Elem;
use fillbar::FillBar;
use super::Widget;
use super::read_program;
use super::super::{Result, Error};

pub struct BatteryW {
    re: regex::Regex,
    fillbars: Vec<FillBar>,
}

#[derive(Clone, Copy)]
enum BatStatus {
    Charging(u64, u64),
    Discharging(u64, u64),
    Full,
}

static COLORS: [(&'static str, &'static str); 6] =
    [
        ("#f42c00", "#a81e00"), // critical battery
        ("#f4db00", "#948500"), // low battery
        ("#51e800", "#2d8200"), // high battery
        ("#00b2f4", "#004290"), // full battery
        ("#0062d7", "#004290"), // charging
        ("#ffffff", "#bbbbbb"), // full + charging
    ];

impl BatteryW {

    pub fn new() -> Self {
        let re = Regex::new(r"Battery \d: (\S+), (\d+)%, (?:(\d+):(\d+):(\d+)|discharging at zero rate)").unwrap();

        let fillbars = COLORS
            .iter()
            .map(|&(bright, dark)| FillBar::gradient(8, dark, bright))
            .collect();

        BatteryW { re, fillbars }
    }

    fn parse_acpi(&self) -> Result<(i32, BatStatus)> {
        let acpi_out = read_program("/bin/acpi")?;

        let caps = self.re.captures(&acpi_out)
                          .ok_or(Error::BadCmdFormat("acpi doesn't match regex"))?;

        let amt = caps[2].parse().unwrap();

        let bat_status =
            match caps.get(3) {
                Some(_) => {
                    let hr = caps[3].parse().unwrap();
                    let mn = caps[4].parse().unwrap();

                    match &caps[1] {
                        "Charging" => BatStatus::Charging(hr, mn),
                        "Discharging" => BatStatus::Discharging(hr, mn),
                        _ => return Err(Error::BadCmdFormat("not charging or discharging?")),
                    }
                }

                None =>
                    BatStatus::Full,
            };

        Ok((amt, bat_status))
    }
}

impl Widget for BatteryW {
    fn cooldown(&self) -> i32 { 6 }

    fn blit(&self) -> Result<Vec<Elem>> {
        use self::BatStatus::*;

        let (amt, bat_status) = self.parse_acpi()?;

        let index = match (bat_status, amt) {
            (Discharging(..),  0 ... 15) => 0,
            (Discharging(..), 15 ... 35) => 1,
            (Discharging(..), 35 ... 95) => 2,
            (Discharging(..), _)         => 3,
            (Charging(..),    _)         => 4,
            (Full,            _)         => 5,
        };

        let chr = match bat_status {
            Discharging(..) => '#',
            _ => '>',
        };

        let time_text = match bat_status {
            Discharging(hr, mn) | Charging(hr, mn) =>
                format!(" {}h{:02}", hr, mn),

            Full => format!("full."),
        };

        let bar = &self.fillbars[index];
        let text_color = COLORS[index].0;

        Ok(iter::once(Elem::plain("bat: ["))
           .chain(bar.blit(chr, amt, 100))
           .chain(vec![Elem::plain("]  "),
                       Elem::new(format!("{:02}% ", amt), text_color),
                       Elem::new(time_text, "#3d3d53")])
           .collect())
    }
}
