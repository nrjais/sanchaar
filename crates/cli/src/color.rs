use std::io;
use std::io::Write;
use std::ops::Add;

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct Weight {
    pub id: i8,
}

impl Color {
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };
    pub const RED: Color = Color {
        r: 255,
        g: 78,
        b: 78,
    };
    pub const ORANGE: Color = Color {
        r: 255,
        g: 158,
        b: 86,
    };
    pub const YELLOW: Color = Color {
        r: 255,
        g: 240,
        b: 86,
    };
    pub const LIGHTGREEN: Color = Color {
        r: 102,
        g: 255,
        b: 124,
    };
    pub const DARKGREEN: Color = Color {
        r: 27,
        g: 141,
        b: 43,
    };
    pub const MINT: Color = Color {
        r: 65,
        g: 255,
        b: 160,
    };
    pub const CYAN: Color = Color {
        r: 74,
        g: 255,
        b: 252,
    };
    pub const LIGHTBLUE: Color = Color {
        r: 88,
        g: 221,
        b: 255,
    };
    pub const SKYBLUE: Color = Color {
        r: 0,
        g: 169,
        b: 255,
    };
    pub const BLUE: Color = Color {
        r: 0,
        g: 91,
        b: 255,
    };
    pub const LIGHTPURPLE: Color = Color {
        r: 0,
        g: 31,
        b: 255,
    };
    pub const DARKBLUE: Color = Color {
        r: 0,
        g: 31,
        b: 255,
    };
    pub const DEEPPURPLE: Color = Color {
        r: 78,
        g: 0,
        b: 255,
    };
    pub const PURPLE: Color = Color {
        r: 123,
        g: 0,
        b: 255,
    };
    pub const VIOLET: Color = Color {
        r: 172,
        g: 108,
        b: 255,
    };
    pub const MAGENTA: Color = Color {
        r: 213,
        g: 0,
        b: 255,
    };
    pub const WARMPINK: Color = Color {
        r: 255,
        g: 0,
        b: 255,
    };
    pub const WATERMELON: Color = Color {
        r: 255,
        g: 113,
        b: 166,
    };
    pub const LIGHTGRAY: Color = Color {
        r: 153,
        g: 153,
        b: 153,
    };
    pub const DARKGRAY: Color = Color {
        r: 91,
        g: 91,
        b: 91,
    };
}

impl Weight {
    pub const BOLD: Weight = Weight { id: 1 };
    pub const DIM: Weight = Weight { id: 2 };
    pub const ITALIC: Weight = Weight { id: 3 };
    pub const UNDERLINE: Weight = Weight { id: 4 };
    pub const SLOWBLINK: Weight = Weight { id: 5 };
    pub const FASTBLINK: Weight = Weight { id: 6 };
}

impl Add<&str> for Color {
    type Output = String;

    fn add(self, rhs: &str) -> Self::Output {
        format!(
            "\x1B[38;2;{};{};{}m{}{}",
            self.r, self.g, self.b, rhs, "\x1B[0m"
        )
    }
}

impl Add<&str> for Weight {
    type Output = String;

    fn add(self, rhs: &str) -> Self::Output {
        return format!("\x1B[{}m{}\x1B[0m", self.id, rhs);
    }
}

pub fn color(text: &str, clr: Color) -> String {
    return clr + text;
}

pub fn weigh(text: &str, wht: Weight) -> String {
    return wht + text;
}

pub fn input_color(iptext: &str, clr: Color) -> String {
    let mut ip = String::new();
    print!("{}\x1B[38;2;{};{};{}m", iptext, clr.r, clr.g, clr.b);
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut ip).expect("Error getting input");
    print!("\x1B[0m");
    return ip.trim_end().to_owned();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!(
            "{}",
            color(
                "red",
                Color {
                    r: 255,
                    g: 255,
                    b: 255
                }
            )
        );
    }
}
