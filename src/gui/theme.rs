use iced::theme::Palette;
use iced::widget::button::{self, Status};
use iced::{Background, Color, Theme};

pub trait PaletteExt {
    fn secondary(&self) -> Color;
    fn tertiary(&self) -> Color;
}

impl PaletteExt for Palette {
    fn secondary(&self) -> Color {
        transform_oklch(self.primary, 0.0, 0.35)
    }
    fn tertiary(&self) -> Color {
        transform_oklch(self.primary, 60.0, 0.70)
    }
}

#[derive(Clone)]
pub enum CrimsonPuppet {
    CrimsonDark,
    CrimsonLight,
}

pub struct ThemeColors {
    pub background: Color,
    pub text: Color,
    pub primary: Color,
    pub success: Color,
    pub danger: Color,
}

impl CrimsonPuppet {
    pub fn get_colors(&self) -> ThemeColors {
        match self {
            CrimsonPuppet::CrimsonDark => ThemeColors {
                background: Color::from_rgb8(0x12, 0x12, 0x14), // Charcoal black
                text: Color::from_rgb8(0xE3, 0xE3, 0xE6),
                primary: Color::from_rgb8(0xDC, 0x14, 0x3C), // Crimson
                success: Color::from_rgb8(0x38, 0x8E, 0x3C),
                danger: Color::from_rgb8(0xD3, 0x2F, 0x2F),
            },
            CrimsonPuppet::CrimsonLight => ThemeColors {
                background: Color::from_rgb8(0xFA, 0xF8, 0xF8), // Warm crisp white
                text: Color::from_rgb8(0x1C, 0x1A, 0x1A),
                primary: Color::from_rgb8(0xA3, 0x00, 0x21), // Deep Crimson
                success: Color::from_rgb8(0x4C, 0xAF, 0x50),
                danger: Color::from_rgb8(0xF4, 0x43, 0x36),
            },
        }
    }

    pub fn to_iced_theme(&self) -> iced::Theme {
        let colors = self.get_colors();
        iced::Theme::custom(
            String::from("CrimsonTheme"),
            Palette {
                background: colors.background,
                text: colors.text,
                primary: colors.primary,
                success: colors.success,
                warning: colors.danger,
                danger: colors.danger,
            },
        )
    }
}

pub fn footer_button_style(theme: &Theme, status: Status) -> button::Style {
    let palette = theme.palette();

    match status {
        Status::Hovered => button::Style {
            background: Some(Background::Color(Color::TRANSPARENT)),
            text_color: Color::inverse(palette.text),
            ..Default::default()
        },
        _ => button::Style {
            background: Some(Background::Color(Color::TRANSPARENT)),
            text_color: palette.text,
            ..Default::default()
        },
    }
}

fn transform_oklch(color: Color, hue_shift_deg: f32, chroma_factor: f32) -> Color {
    // 1. Convert sRGB -> Linear RGB
    let linearize = |v: f32| {
        if v > 0.04045 {
            ((v + 0.055) / 1.055).powf(2.4)
        } else {
            v / 12.92
        }
    };
    let (r, g, b) = (linearize(color.r), linearize(color.g), linearize(color.b));

    // 2. Linear RGB -> LMS cone response
    let l_c = 0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b;
    let m_c = 0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b;
    let s_c = 0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b;

    let (l_, m_, s_) = (l_c.cbrt(), m_c.cbrt(), s_c.cbrt());

    // 3. LMS -> OKLab
    let lightness = 0.2104542553 * l_ + 0.7936177850 * m_ - 0.0040720403 * s_;
    let a = 1.9779984951 * l_ - 2.4285922050 * m_ + 0.4505937099 * s_;
    let b_val = 0.0259040371 * l_ + 0.7827717662 * m_ - 0.8086757993 * s_;

    // 4. OKLab -> Polar OKLCH
    let chroma = (a * a + b_val * b_val).sqrt();
    let mut hue = b_val.atan2(a).to_degrees();
    if hue < 0.0 {
        hue += 360.0;
    }

    // --- APPLY M3 TRANSFORMATIONS ---
    let new_hue = (hue + hue_shift_deg) % 360.0;
    let new_chroma = chroma * chroma_factor;
    // Lightness remains `lightness` (completely untouched!)

    // 5. Convert OKLCH back to OKLab
    let h_rad = new_hue.to_radians();
    let a_new = new_chroma * h_rad.cos();
    let b_new = new_chroma * h_rad.sin();

    // 6. OKLab -> LMS
    let l_back = lightness + 0.3963377774 * a_new + 0.2158037573 * b_new;
    let m_back = lightness - 0.1055613458 * a_new - 0.0638541728 * b_new;
    let s_back = lightness - 0.0894841775 * a_new - 1.2914855480 * b_new;

    let (l_cube, m_cube, s_cube) = (
        l_back * l_back * l_back,
        m_back * m_back * m_back,
        s_back * s_back * s_back,
    );

    // 7. LMS -> Linear RGB
    let r_lin = 4.0767416621 * l_cube - 3.3077115913 * m_cube + 0.2309699292 * s_cube;
    let g_lin = -1.2684380046 * l_cube + 2.6097574011 * m_cube - 0.3413193965 * s_cube;
    let b_lin = -0.0041960863 * l_cube - 0.7034186147 * m_cube + 1.7076147010 * s_cube;

    // 8. Delinearize back to sRGB with clamping
    let delinearize = |v: f32| -> f32 {
        let v_clamped = v.clamp(0.0, 1.0);
        if v_clamped > 0.0031308 {
            1.055 * v_clamped.powf(1.0 / 2.4) - 0.055
        } else {
            12.92 * v_clamped
        }
    };

    Color {
        r: delinearize(r_lin),
        g: delinearize(g_lin),
        b: delinearize(b_lin),
        a: color.a,
    }
}
