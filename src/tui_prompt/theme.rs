use cursive::theme::BaseColor::*;
use cursive::theme::Color::TerminalDefault;
use cursive::theme::Effect::*;
use cursive::theme::PaletteColor::*;
use cursive::theme::Style;
use cursive::theme::{BorderStyle, Palette, Theme};
use cursive::traits::With;

use cock_lib::GetVariants;

pub enum TUIThemes {
    Default,
    DarkSolarized,
    LightSolarized,
    Dracula,
}

impl TUIThemes {
    pub fn get_theme(&self) -> Theme {
        match self {
            TUIThemes::Default => cursive::theme::load_default(),
            TUIThemes::DarkSolarized => cursive::theme::Theme {
                shadow: true,
                borders: BorderStyle::Simple,
                palette: Palette::retro().with(|palette| {
                    {
                        palette[Background] = TerminalDefault;
                        palette[View] = TerminalDefault;
                        palette[Primary] = White.dark();
                        palette[TitlePrimary] = Blue.light();
                        palette[Secondary] = Blue.light();
                        palette[Highlight] = Blue.dark();
                    }

                    {
                        use cursive::theme::PaletteStyle::*;
                        palette[Highlight] = Style::from(Blue.light()).combine(Bold);
                    }
                }),
            },
            TUIThemes::LightSolarized => cursive::theme::Theme {
                shadow: false,
                borders: BorderStyle::Simple,
                palette: Palette::retro().with(|palette| {
                    use cursive::theme::BaseColor::*;

                    {
                        palette[Background] = TerminalDefault;
                        palette[View] = TerminalDefault;
                        palette[Primary] = Cyan.dark();
                        palette[TitlePrimary] = Yellow.light();
                        palette[Secondary] = Yellow.light();
                        palette[Highlight] = Cyan.dark();
                    }

                    {
                        // Then override some styles.
                        use cursive::theme::PaletteStyle::*;
                        palette[Highlight] = Style::from(Yellow.light()).combine(Bold);
                    }
                }),
            },
            TUIThemes::Dracula => cursive::theme::Theme {
                shadow: false,
                borders: BorderStyle::Simple,
                palette: Palette::retro().with(|palette| {
                    use cursive::theme::BaseColor::*;

                    {
                        palette[Background] = TerminalDefault;
                        palette[View] = TerminalDefault;
                        palette[Primary] = Magenta.dark();
                        palette[TitlePrimary] = Cyan.light();
                        palette[Secondary] = Cyan.light();
                        palette[Highlight] = Magenta.dark();
                    }

                    {
                        use cursive::theme::PaletteStyle::*;
                        palette[Highlight] = Style::from(Cyan.light()).combine(Bold);
                    }
                }),
            },
        }
    }

    #[allow(dead_code)]
    pub fn as_str(&self) -> &str {
        match self {
            TUIThemes::Default => "Default",
            TUIThemes::DarkSolarized => "Dark Solarized",
            TUIThemes::LightSolarized => "Light Solarized",
            TUIThemes::Dracula => "Dracula",
        }
    }
}

impl GetVariants for TUIThemes {
    fn get_variants() -> Vec<String> {
        vec![
            "Default".to_string(),
            "Dark Solarized".to_string(),
            "Light Solarized".to_string(),
            "Dracula".to_string(),
        ]
    }
}

