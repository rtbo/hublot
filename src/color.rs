use std::collections::HashMap;
use std::fmt;

/// Color represented with ARGB 8 bits per channel
#[derive(Copy, Clone)]
pub struct Color {
    col: u32,
}

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Color {{ R: {:02x}, G: {:02x}, B: {:02x}, A: {:02x} }}",
            self.red(),
            self.green(),
            self.blue(),
            self.alpha()
        )
    }
}

impl Color {
    #[inline]
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color {
            col: (r as u32).rotate_left(16)
                | (g as u32).rotate_left(8)
                | (b as u32)
                | (a as u32).rotate_left(24),
        }
    }
    #[inline]
    pub fn fnew(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color::new(
            (r * 255f32) as u8,
            (g * 255f32) as u8,
            (b * 255f32) as u8,
            (a * 255f32) as u8,
        )
    }
    pub fn from_name<S: AsRef<str>>(name: S) -> Option<Color> {
        CSS_NAMES.get(name.as_ref()).map(|&name| Color::from(name))
    }
    #[inline]
    pub fn red(&self) -> u8 {
        (self.col & 0x00ff0000).rotate_right(16) as u8
    }
    #[inline]
    pub fn green(&self) -> u8 {
        (self.col & 0x0000ff00).rotate_right(8) as u8
    }
    #[inline]
    pub fn blue(&self) -> u8 {
        (self.col & 0x000000ff) as u8
    }
    #[inline]
    pub fn alpha(&self) -> u8 {
        (self.col & 0xff000000).rotate_right(24) as u8
    }
    #[inline]
    pub fn fred(&self) -> f32 {
        self.red() as f32 / 255f32
    }
    #[inline]
    pub fn fgreen(&self) -> f32 {
        self.green() as f32 / 255f32
    }
    #[inline]
    pub fn fblue(&self) -> f32 {
        self.blue() as f32 / 255f32
    }
    #[inline]
    pub fn falpha(&self) -> f32 {
        self.alpha() as f32 / 255f32
    }
}

impl From<u32> for Color {
    fn from(val: u32) -> Color {
        Color { col: val }
    }
}

impl From<[u8; 4]> for Color {
    fn from(val: [u8; 4]) -> Color {
        Color::new(val[0], val[1], val[2], val[3])
    }
}

impl From<[f32; 4]> for Color {
    fn from(val: [f32; 4]) -> Color {
        Color::fnew(val[0], val[1], val[2], val[3])
    }
}

impl From<CssName> for Color {
    fn from(val: CssName) -> Color {
        Color { col: val as u32 }
    }
}

impl From<Color> for u32 {
    fn from(val: Color) -> u32 {
        val.col
    }
}

impl From<Color> for [u8; 4] {
    fn from(val: Color) -> [u8; 4] {
        [val.red(), val.green(), val.blue(), val.alpha()]
    }
}

impl From<Color> for [f32; 4] {
    fn from(val: Color) -> [f32; 4] {
        [val.fred(), val.fgreen(), val.fblue(), val.falpha()]
    }
}

/// Standards: https://www.w3.org/TR/css3-color/#svg-color
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum CssName {
    Transparent = 0x00000000,

    AliceBlue = 0xfff0f8ff,
    AntiqueWhite = 0xfffaebd7,
    AquaMarine = 0xff7fffd4,
    Azure = 0xfff0ffff,
    Beige = 0xfff5f5dc,
    Bisque = 0xffffe4c4,
    Black = 0xff000000,
    BlanchedAlmond = 0xffffebcd,
    Blue = 0xff0000ff,
    BlueViolet = 0xff8a2be2,
    Brown = 0xffa52a2a,
    BurlyWood = 0xffdeb887,
    CadetBlue = 0xff5f9ea0,
    Chartreuse = 0xff7fff00,
    Chocolate = 0xffd2691e,
    Coral = 0xffff7f50,
    CornFlowerBlue = 0xff6495ed,
    CornSilk = 0xfffff8dc,
    Crimson = 0xffdc143c,
    Cyan = 0xff00ffff,
    DarkBlue = 0xff00008b,
    DarkCyan = 0xff008b8b,
    DarkGoldenrod = 0xffb8860b,
    DarkGreen = 0xff006400,
    DarkGrey = 0xffa9a9a9,
    DarkKhaki = 0xffbdb76b,
    DarkMagenta = 0xff8b008b,
    DarkOliveGreen = 0xff556b2f,
    DarkOrange = 0xffff8c00,
    DarkOrchid = 0xff9932cc,
    DarkRed = 0xff8b0000,
    DarkSalmon = 0xffe9967a,
    DarkSeaGreen = 0xff8fbc8f,
    DarkSlateBlue = 0xff483d8b,
    DarkSlateGrey = 0xff2f4f4f,
    DarkTurquoise = 0xff00ced1,
    DarkViolet = 0xff9400d3,
    DeepPink = 0xffff1493,
    DeepSkyBlue = 0xff00bfff,
    DimGrey = 0xff696969,
    DodgerBlue = 0xff1e90ff,
    FireBrick = 0xffb22222,
    FloralWhite = 0xfffffaf0,
    ForestGreen = 0xff228b22,
    Gainsboro = 0xffdcdcdc,
    GhostWhite = 0xfff8f8ff,
    Gold = 0xffffd700,
    Goldenrod = 0xffdaa520,
    Green = 0xff008000,
    GreenYellow = 0xffadff2f,
    Grey = 0xff808080,
    HoneyDew = 0xfff0fff0,
    HotPink = 0xffff69b4,
    IndianRed = 0xffcd5c5c,
    Indigo = 0xff4b0082,
    Ivory = 0xfffffff0,
    Khaki = 0xfff0e68c,
    Lavender = 0xffe6e6fa,
    LavenderBlush = 0xfffff0f5,
    LawnGreen = 0xff7cfc00,
    LemonChiffon = 0xfffffacd,
    LightBlue = 0xffadd8e6,
    LightCoral = 0xfff08080,
    LightCyan = 0xffe0ffff,
    LightGoldenrodYellow = 0xfffafad2,
    LightGreen = 0xff90ee90,
    LightGrey = 0xffd3d3d3,
    LightPink = 0xffffb6c1,
    LightSalmon = 0xffffa07a,
    LightSeaGreen = 0xff20b2aa,
    LightSkyBlue = 0xff87cefa,
    LightSlateGrey = 0xff778899,
    LightSteelBlue = 0xffb0c4de,
    LightYellow = 0xffffffe0,
    Lime = 0xff00ff00,
    LimeGreen = 0xff32cd32,
    Linen = 0xfffaf0e6,
    Magenta = 0xffff00ff,
    Maroon = 0xff800000,
    MediumAquaMarine = 0xff66cdaa,
    MediumBlue = 0xff0000cd,
    MediumOrchid = 0xffba55d3,
    MediumPurple = 0xff9370db,
    MediumSeaGreen = 0xff3cb371,
    MediumSlateBlue = 0xff7b68ee,
    MediumSpringGreen = 0xff00fa9a,
    MediumTurquoise = 0xff48d1cc,
    MediumVioletRed = 0xffc71585,
    MidnightBlue = 0xff191970,
    MintCream = 0xfff5fffa,
    MistyRose = 0xffffe4e1,
    Moccasin = 0xffffe4b5,
    NavajoWhite = 0xffffdead,
    Navy = 0xff000080,
    OldLace = 0xfffdf5e6,
    Olive = 0xff808000,
    OliveDrab = 0xff6b8e23,
    Orange = 0xffffa500,
    OrangeRed = 0xffff4500,
    Orchid = 0xffda70d6,
    PaleGoldenrod = 0xffeee8aa,
    PaleGreen = 0xff98fb98,
    PaleTurquoise = 0xffafeeee,
    PaleVioletRed = 0xffdb7093,
    PapayaWhip = 0xffffefd5,
    PeachPuff = 0xffffdab9,
    Peru = 0xffcd853f,
    Pink = 0xffffc0cb,
    Plum = 0xffdda0dd,
    PowderBlue = 0xffb0e0e6,
    Purple = 0xff800080,
    Red = 0xffff0000,
    RosyBrown = 0xffbc8f8f,
    RoyalBlue = 0xff4169e1,
    SaddleBrown = 0xff8b4513,
    Salmon = 0xfffa8072,
    SandyBrown = 0xfff4a460,
    SeaGreen = 0xff2e8b57,
    SeaShell = 0xfffff5ee,
    Sienna = 0xffa0522d,
    Silver = 0xffc0c0c0,
    SkyBlue = 0xff87ceeb,
    SlateBlue = 0xff6a5acd,
    SlateGrey = 0xff708090,
    Snow = 0xfffffafa,
    SpringGreen = 0xff00ff7f,
    SteelBlue = 0xff4682b4,
    Tan = 0xffd2b48c,
    Teal = 0xff008080,
    Thistle = 0xffd8bfd8,
    Tomato = 0xffff6347,
    Turquoise = 0xff40e0d0,
    Violet = 0xffee82ee,
    Wheat = 0xfff5deb3,
    White = 0xffffffff,
    WhiteSmoke = 0xfff5f5f5,
    Yellow = 0xffffff00,
    YellowGreen = 0xff9acd32,
}

lazy_static! {
    static ref CSS_NAMES: HashMap<&'static str, CssName> = {
        let mut m = HashMap::new();
        m.insert("aliceblue", CssName::AliceBlue);
        m.insert("antiquewhite", CssName::AntiqueWhite);
        m.insert("aqua", CssName::Cyan);
        m.insert("aquamarine", CssName::AquaMarine);
        m.insert("azure", CssName::Azure);
        m.insert("beige", CssName::Beige);
        m.insert("bisque", CssName::Bisque);
        m.insert("black", CssName::Black);
        m.insert("blanchedalmond", CssName::BlanchedAlmond);
        m.insert("blue", CssName::Blue);
        m.insert("blueviolet", CssName::BlueViolet);
        m.insert("brown", CssName::Brown);
        m.insert("burlywood", CssName::BurlyWood);
        m.insert("cadetblue", CssName::CadetBlue);
        m.insert("chartreuse", CssName::Chartreuse);
        m.insert("chocolate", CssName::Chocolate);
        m.insert("coral", CssName::Coral);
        m.insert("cornflowerblue", CssName::CornFlowerBlue);
        m.insert("cornsilk", CssName::CornSilk);
        m.insert("crimson", CssName::Crimson);
        m.insert("cyan", CssName::Cyan);
        m.insert("darkblue", CssName::DarkBlue);
        m.insert("darkcyan", CssName::DarkCyan);
        m.insert("darkgoldenrod", CssName::DarkGoldenrod);
        m.insert("darkgray", CssName::DarkGrey);
        m.insert("darkgreen", CssName::DarkGreen);
        m.insert("darkgrey", CssName::DarkGrey);
        m.insert("darkkhaki", CssName::DarkKhaki);
        m.insert("darkmagenta", CssName::DarkMagenta);
        m.insert("darkolivegreen", CssName::DarkOliveGreen);
        m.insert("darkorange", CssName::DarkOrange);
        m.insert("darkorchid", CssName::DarkOrchid);
        m.insert("darkred", CssName::DarkRed);
        m.insert("darksalmon", CssName::DarkSalmon);
        m.insert("darkseagreen", CssName::DarkSeaGreen);
        m.insert("darkslateblue", CssName::DarkSlateBlue);
        m.insert("darkslategray", CssName::DarkSlateGrey);
        m.insert("darkslategrey", CssName::DarkSlateGrey);
        m.insert("darkturquoise", CssName::DarkTurquoise);
        m.insert("darkviolet", CssName::DarkViolet);
        m.insert("deeppink", CssName::DeepPink);
        m.insert("deepskyblue", CssName::DeepSkyBlue);
        m.insert("dimgray", CssName::DimGrey);
        m.insert("dimgrey", CssName::DimGrey);
        m.insert("dodgerblue", CssName::DodgerBlue);
        m.insert("firebrick", CssName::FireBrick);
        m.insert("floralwhite", CssName::FloralWhite);
        m.insert("forestgreen", CssName::ForestGreen);
        m.insert("fuchsia", CssName::Magenta);
        m.insert("gainsboro", CssName::Gainsboro);
        m.insert("ghostwhite", CssName::GhostWhite);
        m.insert("gold", CssName::Gold);
        m.insert("goldenrod", CssName::Goldenrod);
        m.insert("green", CssName::Green);
        m.insert("greenyellow", CssName::GreenYellow);
        m.insert("gray", CssName::Grey);
        m.insert("grey", CssName::Grey);
        m.insert("honeydew", CssName::HoneyDew);
        m.insert("hotpink", CssName::HotPink);
        m.insert("indianred", CssName::IndianRed);
        m.insert("indigo", CssName::Indigo);
        m.insert("ivory", CssName::Ivory);
        m.insert("khaki", CssName::Khaki);
        m.insert("lavender", CssName::Lavender);
        m.insert("lavenderblush", CssName::LavenderBlush);
        m.insert("lawngreen", CssName::LawnGreen);
        m.insert("lemonchiffon", CssName::LemonChiffon);
        m.insert("lightblue", CssName::LightBlue);
        m.insert("lightcoral", CssName::LightCoral);
        m.insert("lightcyan", CssName::LightCyan);
        m.insert("lightgoldenrodyellow", CssName::LightGoldenrodYellow);
        m.insert("lightgray", CssName::LightGrey);
        m.insert("lightgreen", CssName::LightGreen);
        m.insert("lightgrey", CssName::LightGrey);
        m.insert("lightpink", CssName::LightPink);
        m.insert("lightsalmon", CssName::LightSalmon);
        m.insert("lightseagreen", CssName::LightSeaGreen);
        m.insert("lightskyblue", CssName::LightSkyBlue);
        m.insert("lightslategray", CssName::LightSlateGrey);
        m.insert("lightslategrey", CssName::LightSlateGrey);
        m.insert("lightsteelblue", CssName::LightSteelBlue);
        m.insert("lightyellow", CssName::LightYellow);
        m.insert("lime", CssName::Lime);
        m.insert("limegreen", CssName::LimeGreen);
        m.insert("linen", CssName::Linen);
        m.insert("magenta", CssName::Magenta);
        m.insert("maroon", CssName::Maroon);
        m.insert("mediumaquamarine", CssName::MediumAquaMarine);
        m.insert("mediumblue", CssName::MediumBlue);
        m.insert("mediumorchid", CssName::MediumOrchid);
        m.insert("mediumpurple", CssName::MediumPurple);
        m.insert("mediumseagreen", CssName::MediumSeaGreen);
        m.insert("mediumslateblue", CssName::MediumSlateBlue);
        m.insert("mediumspringgreen", CssName::MediumSpringGreen);
        m.insert("mediumturquoise", CssName::MediumTurquoise);
        m.insert("mediumvioletred", CssName::MediumVioletRed);
        m.insert("midnightblue", CssName::MidnightBlue);
        m.insert("mintcream", CssName::MintCream);
        m.insert("mistyrose", CssName::MistyRose);
        m.insert("moccasin", CssName::Moccasin);
        m.insert("navajowhite", CssName::NavajoWhite);
        m.insert("navy", CssName::Navy);
        m.insert("oldlace", CssName::OldLace);
        m.insert("olive", CssName::Olive);
        m.insert("olivedrab", CssName::OliveDrab);
        m.insert("orange", CssName::Orange);
        m.insert("orangered", CssName::OrangeRed);
        m.insert("orchid", CssName::Orchid);
        m.insert("palegoldenrod", CssName::PaleGoldenrod);
        m.insert("palegreen", CssName::PaleGreen);
        m.insert("paleturquoise", CssName::PaleTurquoise);
        m.insert("palevioletred", CssName::PaleVioletRed);
        m.insert("papayawhip", CssName::PapayaWhip);
        m.insert("peachpuff", CssName::PeachPuff);
        m.insert("peru", CssName::Peru);
        m.insert("pink", CssName::Pink);
        m.insert("plum", CssName::Plum);
        m.insert("powderblue", CssName::PowderBlue);
        m.insert("purple", CssName::Purple);
        m.insert("red", CssName::Red);
        m.insert("rosybrown", CssName::RosyBrown);
        m.insert("royalblue", CssName::RoyalBlue);
        m.insert("saddlebrown", CssName::SaddleBrown);
        m.insert("salmon", CssName::Salmon);
        m.insert("sandybrown", CssName::SandyBrown);
        m.insert("seagreen", CssName::SeaGreen);
        m.insert("seashell", CssName::SeaShell);
        m.insert("sienna", CssName::Sienna);
        m.insert("silver", CssName::Silver);
        m.insert("skyblue", CssName::SkyBlue);
        m.insert("slateblue", CssName::SlateBlue);
        m.insert("slategray", CssName::SlateGrey);
        m.insert("slategrey", CssName::SlateGrey);
        m.insert("snow", CssName::Snow);
        m.insert("springgreen", CssName::SpringGreen);
        m.insert("steelblue", CssName::SteelBlue);
        m.insert("tan", CssName::Tan);
        m.insert("teal", CssName::Teal);
        m.insert("thistle", CssName::Thistle);
        m.insert("tomato", CssName::Tomato);
        m.insert("turquoise", CssName::Turquoise);
        m.insert("violet", CssName::Violet);
        m.insert("wheat", CssName::Wheat);
        m.insert("white", CssName::White);
        m.insert("whitesmoke", CssName::WhiteSmoke);
        m.insert("yellow", CssName::Yellow);
        m.insert("yellowgreen", CssName::YellowGreen);
        m.shrink_to_fit();
        m
    };
}
