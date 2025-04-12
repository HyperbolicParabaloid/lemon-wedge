pub enum FontSize {
    XXS,
    XS,
    S,
    M,
    L,
    XL,
    XXL
}

impl FontSize {
    pub const fn as_f32(&self) -> f32 {
        match self {
            FontSize::XXS   => 1.0 / 2048.0,
            FontSize::XS    => 1.0 / 1024.0,
            FontSize::S     => 1.0 / 512.0,
            FontSize::M     => 1.0 / 128.0,
            FontSize::L     => 1.0 / 64.0,
            FontSize::XL    => 1.0 / 32.0,
            FontSize::XXL   => 1.0 / 16.0,
        }
    }
}
