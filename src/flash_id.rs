//! Flash ID parsing.

/// Flash ID manufacturer code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Manufacturer {
    /// Spansion.
    Spansion = 0x1,
    /// Fujitsu.
    Fujitsu = 0x4,
    /// Renesas (Hitachi).
    Renesas = 0x7,
    /// `STMicroelectronics` / Numonyx.
    StMicro = 0x20,
    /// Micron.
    Micron = 0x2C,
    /// `SanDisk`.
    SanDisk = 0x45,
    /// SMIC.
    Smic = 0x4A,
    /// Qimonda.
    Qimonda = 0x51,
    /// Intel.
    Intel = 0x89,
    /// National Semiconductor.
    National = 0x8F,
    /// ESMT (`PowerChip`).
    EsmtPowerchip = 0x92,
    /// Kioxia (Toshiba).
    Kioxia = 0x98,
    /// YMTC (Yangtze Memory).
    Ymtc = 0x9B,
    /// ISSI (Integrated Silicon Solution).
    Issi = 0x9D,
    /// SK Hynix.
    SkHynix = 0xAD,
    /// `SpecTek` (Micron).
    SpecTek = 0xB5,
    /// Macronix.
    Macronix = 0xC2,
    /// ESMT (MIRA / PSC).
    EsmtMiraPsc = 0xC8,
    /// Dosilicon.
    Dosilicon = 0xE5,
    /// Samsung.
    Samsung = 0xEC,
    /// Winbond.
    Winbond = 0xEF,
}

impl TryFrom<u8> for Manufacturer {
    /// The unrecognised byte is returned verbatim.
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        const VARIANTS: &[Manufacturer] = &[
            Manufacturer::Spansion,
            Manufacturer::Fujitsu,
            Manufacturer::Renesas,
            Manufacturer::StMicro,
            Manufacturer::Micron,
            Manufacturer::SanDisk,
            Manufacturer::Smic,
            Manufacturer::Qimonda,
            Manufacturer::Intel,
            Manufacturer::National,
            Manufacturer::EsmtPowerchip,
            Manufacturer::Kioxia,
            Manufacturer::Ymtc,
            Manufacturer::Issi,
            Manufacturer::SkHynix,
            Manufacturer::SpecTek,
            Manufacturer::Macronix,
            Manufacturer::EsmtMiraPsc,
            Manufacturer::Dosilicon,
            Manufacturer::Samsung,
            Manufacturer::Winbond,
        ];

        VARIANTS
            .iter()
            .find(|&&x| x as u8 == value)
            .copied()
            .ok_or(value)
    }
}

impl std::fmt::Display for Manufacturer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Spansion => "Spansion",
            Self::Fujitsu => "Fujitsu",
            Self::Renesas => "Renesas",
            Self::StMicro => "STMicro",
            Self::Micron => "Micron",
            Self::SanDisk => "SanDisk",
            Self::Smic => "SMIC",
            Self::Qimonda => "Qimonda",
            Self::Intel => "Intel",
            Self::National => "National Semiconductor",
            Self::EsmtPowerchip => "ESMT (PowerChip)",
            Self::Kioxia => "Kioxia (Toshiba)",
            Self::Ymtc => "YMTC",
            Self::Issi => "ISSI",
            Self::SkHynix => "SK Hynix",
            Self::SpecTek => "SpecTek",
            Self::Macronix => "Macronix",
            Self::EsmtMiraPsc => "ESMT (MIRA/PSC)",
            Self::Dosilicon => "Dosilicon",
            Self::Samsung => "Samsung",
            Self::Winbond => "Winbond",
        })
    }
}
