use super::Board;
use core::fmt;

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in (0..9).rev() {
            for x in 0..9 {
                write!(
                    f,
                    "{}",
                    match self.get(x, y) {
                        Some(v) => v.to_string(),
                        None => " ".to_string(),
                    }
                )?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
