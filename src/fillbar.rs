use std;
use render::Elem;

// a visual bar (e.g. [===    ])
#[derive(Clone)]
pub struct FillBar {
    colors: Vec<RGB>,
}

// color with red, green, blue components (0 <= r,g,b <= 255)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct RGB {
    r: i32, g: i32, b: i32,
}

//
impl FillBar {
    pub fn single<C: AsRef<str>>(len: i32, c: C) -> Self {
        let color = c.as_ref().parse().unwrap();
        let colors = vec![color; len as usize];
        FillBar { colors }
    }

    pub fn gradient<C1, C2>(len: i32, c1: C1, c2: C2) -> Self
        where C1: AsRef<str>,
              C2: AsRef<str> {

        let color1 = c1.as_ref().parse().unwrap();
        let color2 = c2.as_ref().parse().unwrap();
        let colors = (0..len)
            .map(|i| RGB::lerp(color1, color2, i, len - 1))
            .collect();

        FillBar { colors }
    }

    pub fn len(&self) -> usize {
        self.colors.len()
    }

    pub fn blit(&self, chr: char, amt: i32, tot: i32) -> Vec<Elem> {
        use std::iter;
        use std::cmp;

        let midp = self.len() as i32 * amt / tot;
        let nfilled = cmp::min((midp + 1) as usize, self.len());
        let nblank = self.len() - nfilled;

        let single_char_elem = |color: RGB| {
            Elem {
                text: iter::once(chr).collect(),
                color: color.to_string(),
            }
        };

        let filled =
            self.colors
                .iter()
                .cloned()
                .take(nfilled)
                .map(single_char_elem);

        let blank = iter::once(Elem {
            text: iter::repeat(' ').take(nblank).collect(),
            color: "#000000".into(),
        });

        filled.chain(blank).collect()
    }
}



impl RGB {
    // hex string representation of color
    fn to_string(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }

    // linear interpolation based on fraction p/q  (0 <= p <= q)
    fn lerp(c1: RGB, c2: RGB, p: i32, q: i32) -> RGB {
        RGB {
            r: c1.r + (c2.r - c1.r) * p / q,
            g: c1.g + (c2.g - c1.g) * p / q,
            b: c1.b + (c2.b - c1.b) * p / q,
        }
    }
}


#[derive(Debug, Eq, PartialEq)]
struct ParseRGBError;

// parse RGB color from hex string (#RRGGBB)
impl std::str::FromStr for RGB {
    type Err = ParseRGBError;

    fn from_str(s: &str) -> std::result::Result<Self, ParseRGBError> {
        if s.len() != 7 || !s.starts_with('#') {
            return Err(ParseRGBError)
        }

        let n = i32::from_str_radix(&s[1..], 16).map_err(|_| ParseRGBError)?;

        Ok(RGB {
            r: n >> 16,
            g: (n >> 8) & 0xff,
            b: n & 0xff,
        })
    }
}




#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn rgb_conv_test() {
        assert_eq!("#123456".parse(), Ok(RGB { r: 0x12, g: 0x34, b: 0x56 }));
        assert_eq!((RGB { r: 0x12, g: 0x34, b: 0x56 }).to_string(), "#123456");
    }
}
