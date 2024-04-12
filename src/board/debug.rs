use super::Board;
use core::fmt;

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in (0..9).rev() {
            for x in 0..9 {
                write!(f, "{}", {
                    let v = self.get(x, y);
                    match v {
                        0 => " ".to_string(),
                        _ => v.to_string(),
                    }
                })?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
