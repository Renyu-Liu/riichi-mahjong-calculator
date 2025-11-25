use iced::Color;
use iced::widget::button;

pub struct ColoredButtonStyle {
    pub background_color: Color,
    pub text_color: Color,
}

impl button::StyleSheet for ColoredButtonStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(self.background_color)),
            text_color: self.text_color,
            border: iced::Border::with_radius(7.0),
            ..Default::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
        button::Appearance {
            background: Some(iced::Background::Color(Color {
                a: 0.8,
                ..self.background_color
            })),
            ..active
        }
    }
}

pub struct OverlayStyle;

impl iced::widget::container::StyleSheet for OverlayStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        iced::widget::container::Appearance {
            background: Some(iced::Background::Color(Color {
                a: 0.95,
                ..Color::BLACK
            })),
            ..Default::default()
        }
    }
}
