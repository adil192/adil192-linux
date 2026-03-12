use cosmic::{
    cosmic_config::CosmicConfigEntry,
    cosmic_theme::{
        Theme,
        palette::{self, blend::Compose},
    },
};

#[unsafe(no_mangle)]
pub extern "C" fn get_theme(is_dark: bool) -> CosmicThemeFfi {
    let theme = _get_theme(is_dark);
    let ffi = CosmicThemeFfi::from(&theme);
    ffi
}

fn _get_theme(is_dark: bool) -> Theme {
    let config = if is_dark {
        Theme::dark_config()
    } else {
        Theme::light_config()
    };
    let theme = match config {
        Ok(config) => match Theme::get_entry(&config) {
            Ok(theme) => theme,
            Err((_errors, theme)) => theme,
        },
        Err(_) => {
            if is_dark {
                Theme::dark_default()
            } else {
                Theme::light_default()
            }
        }
    };
    theme
}

#[repr(C)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[repr(C)]
pub struct CosmicThemeFfi {
    pub background_base: Rgb,
    pub background_on: Rgb,
    pub component_base: Rgb,
    pub component_on: Rgb,
    pub button_base: Rgb,
    pub button_on: Rgb,
}

impl From<&Theme> for CosmicThemeFfi {
    fn from(theme: &Theme) -> Self {
        let base = theme.background.base;
        Self {
            background_base: encode(theme.background.base, base),
            background_on: encode(theme.background.on, base),
            component_base: encode(theme.background.component.base, base),
            component_on: encode(theme.background.component.on, base),
            button_base: encode(theme.button.base, base),
            button_on: encode(theme.button.on, base),
        }
    }
}

fn encode(c: palette::Srgba, base: palette::Srgba) -> Rgb {
    let c_u8: palette::rgb::Rgba<palette::encoding::Srgb, u8> = c.over(base).into_format();
    Rgb {
        r: c_u8.red,
        g: c_u8.green,
        b: c_u8.blue,
    }
}
