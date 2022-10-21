pub struct Rose {
    base: String,
    surface: String,
    overlay: String,
    muted: String,
    subtle: String,
    text: String,
    love: String,
    gold: String,
    rose: String,
    pine: String,
    foam: String,
    iris: String,
    hllow: String,
    hlmed: String,
    hlhigh: String,
}

pub enum Colors {
    Base,
    Surface,
    Overlay,
    Muted,
    Subtle,
    Text,
    Love,
    Gold,
    Rose,
    Pine,
    Foam,
    Iris,
    HLLow,
    HLMed,
    HLHigh,
}
pub enum Palletes {
    Default,
    Moon,
    Dawn,
}

pub enum ColorFormat {
    RGBHex,
    // i have no idea what this is called!
    UnixHex,
}

impl Rose {
    pub fn build(pallete: Palletes) -> Rose {
        match pallete {
            Palletes::Default => Rose {
                base: String::from("191724"),
                surface: String::from("1f1d2e"),
                overlay: String::from("26233a"),
                muted: String::from("6e6a86"),
                subtle: String::from("908caa"),
                text: String::from("e0def4"),
                love: String::from("eb6f92"),
                gold: String::from("f6c177"),
                rose: String::from("ebbcba"),
                pine: String::from("31748f"),
                foam: String::from("9ccfd8"),
                iris: String::from("c4a7e7"),
                hllow: String::from("21202e"),
                hlmed: String::from("403d52"),
                hlhigh: String::from("524f67"),
            },
            Palletes::Moon => todo!(),
            Palletes::Dawn => todo!(),
        }
    }

    pub fn get(&self, color: Colors, format: ColorFormat) -> String {
        let prefix = match format {
            ColorFormat::RGBHex => "#".to_owned(),
            ColorFormat::UnixHex => "0x".to_owned(),
        };

        match color {
            Colors::Base => prefix + &self.base,
            Colors::Surface => prefix + &self.surface,
            Colors::Overlay => prefix + &self.overlay,
            Colors::Muted => prefix + &self.muted,
            Colors::Subtle => prefix + &self.subtle,
            Colors::Text => prefix + &self.text,
            Colors::Love => prefix + &self.love,
            Colors::Gold => prefix + &self.gold,
            Colors::Rose => prefix + &self.rose,
            Colors::Pine => prefix + &self.pine,
            Colors::Foam => prefix + &self.foam,
            Colors::Iris => prefix + &self.iris,
            Colors::HLLow => prefix + &self.hllow,
            Colors::HLMed => prefix + &self.hlmed,
            Colors::HLHigh => prefix + &self.hlhigh,
        }
    }
}
