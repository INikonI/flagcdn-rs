use std::fmt;

pub(crate) trait Size: fmt::Display {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WavingSize {
    _16x12,
    _20x15,
    _24x18,
    _28x21,
    _32x24,
    _36x27,
    _40x30,
    _48x36,
    _56x42,
    _60x45,
    _64x48,
    _72x54,
    _80x60,
    _84x63,
    _96x72,
    _108x81,
    _112x84,
    _120x90,
    _128x96,
    _144x108,
    _160x120,
    _192x144,
    _224x168,
    _256x192,
}

impl fmt::Display for WavingSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::_16x12 => "16x12",
            Self::_20x15 => "20x15",
            Self::_24x18 => "24x18",
            Self::_28x21 => "28x21",
            Self::_32x24 => "32x24",
            Self::_36x27 => "36x27",
            Self::_40x30 => "40x30",
            Self::_48x36 => "48x36",
            Self::_56x42 => "56x42",
            Self::_60x45 => "60x45",
            Self::_64x48 => "64x48",
            Self::_72x54 => "72x54",
            Self::_80x60 => "80x60",
            Self::_84x63 => "84x63",
            Self::_96x72 => "96x72",
            Self::_108x81 => "108x81",
            Self::_112x84 => "112x84",
            Self::_120x90 => "120x90",
            Self::_128x96 => "128x96",
            Self::_144x108 => "144x108",
            Self::_160x120 => "160x120",
            Self::_192x144 => "192x144",
            Self::_224x168 => "224x168",
            Self::_256x192 => "256x192",
        })
    }
}

impl Size for WavingSize {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FixedHeight {
    XS = 20,
    S = 24,
    M = 40,
    L = 60,
    XL = 80,
    XXL = 120,
    XXXL = 240,
}

impl fmt::Display for FixedHeight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "h{}", *self as u8)
    }
}

impl Size for FixedHeight {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum FixedWidth {
    XXS = 20,
    XS = 40,
    S = 80,
    M = 160,
    L = 320,
    XL = 640,
    XXL = 1280,
    XXXL = 2560,
}

impl fmt::Display for FixedWidth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "w{}", *self as u16)
    }
}

impl Size for FixedWidth {}
