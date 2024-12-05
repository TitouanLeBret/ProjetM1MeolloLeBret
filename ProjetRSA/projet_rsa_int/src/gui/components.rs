// Modules nécessaire pour les composant
use iced::theme::Theme;
use iced::widget::{button, container, text, Button, Container, Row, TextInput};
use iced:: { Alignment, Background, Shadow, Vector, Border, Padding,Length};
use iced::Color;
use iced::widget::text_input::StyleSheet;
use iced::widget::text_input::Appearance;
use iced::widget::text_input;

use super::gui;

/// Crée un pied de page contenant une liste de boutons.
///
/// # Arguments
/// 
/// * `btns` - Un vecteur de boutons à inclure dans le pied de page.
///
/// # Retourne
/// 
/// Un conteneur aligné au centre contenant les boutons.
pub fn page_footer(btns : Vec<Button<gui::Message>>) -> Container<gui::Message> {
    let mut footer = Row::new()
        .push( 
            button("Toggle Theme")
                .on_press(gui::Message::ToggleTheme)
                .style(
                    iced::theme::Button::Custom(Box::new(gui::ButtonStyle::ThemeButton)),
                ),
        )
        .align_items(Alignment::Center)
        .spacing(10);
    
    for btn in btns {
        footer = footer.push(btn);
    }

    container(footer).center_x().center_y()

}



/// Style personnalisé pour les conteneurs.
pub struct ContainerStyle;

impl container::StyleSheet for ContainerStyle {
    type Style = Theme;


    /// Définit l'apparence des conteneurs.
    fn appearance(&self , _theme: &Self::Style) -> container::Appearance {
        container::Appearance{
            text_color : Default::default(),
            border: Border::with_radius(5),
            background: None,
            shadow: Shadow {
                color: iced::Color::BLACK,
                offset: Vector::new(0.0,2.0),
                blur_radius: 40.0,
            },
        }
    }
}


/// Structure personnalisé pour les champs de texte.
pub struct MyTextInput ;

impl MyTextInput {
    /// Crée une instance du style `MyTextInput`.
    fn create() -> Self{
        MyTextInput
    }


    /// Crée un champ de texte avec des paramètres personnalisés.
    ///
    /// # Arguments
    /// 
    /// * `_placeholder` - Texte d'espace réservé (placeholder).
    /// * `_value` - Valeur initiale du champ.
    ///
    /// # Retourne
    ///
    /// Un champ de texte stylisé.
    pub fn new( _placeholder:&str,_value : &str,) -> TextInput<'static, gui::Message> {
        TextInput::new(_placeholder,_value)
            .line_height(text::LineHeight::Relative(1.75))
            .padding(Padding::from(12))
            .width(Length::Fixed(300.0))
            .style(iced::theme::TextInput::Custom(Box::new(MyTextInput::create())))
    }
}


///Style de notre structure MyTextInput
impl StyleSheet for MyTextInput {
    type Style = Theme;

    /// Style actif (par défaut) pour le champ de texte.
    fn active(&self, _: &Self::Style) -> Appearance {
        text_input::Appearance {
            background: Background::Color(iced::Color::from_rgb8(5, 11, 31)),
            border: Border::with_radius(10),
            icon_color: iced::Color::from_rgb(0.8, 0.8, 0.8),
        }
    }

    /// Style du champ de texte lorsqu'il est sélectionné.
    fn focused(&self, _: &Self::Style) -> Appearance {
        text_input::Appearance {
            background: Background::Color(iced::Color::from_rgb8(5, 11, 51)),
            border: Border::with_radius(10),
            icon_color: iced::Color::from_rgb(0.8, 0.8, 0.8),
        }
    }

    /// Couleur du placeholder (texte indicatif).
    fn placeholder_color(&self, _: &Self::Style) -> Color {
        Color::from_rgb8(70, 70, 70)
    }

    /// Couleur du texte saisi.
    fn value_color(&self, _: &Self::Style) -> Color {
        Color::from_rgb8(255, 255, 255)
    }

    /// Couleur de sélection dans le champ de texte.
    fn selection_color(&self, _: &Self::Style) -> Color {
        Color::from_rgb(0.8, 0.8, 1.0)
    }

    /// Couleur du texte lorsqu'il est désactivé.
    fn disabled_color(&self, _: &Self::Style) -> Color {
        Color::from_rgb(0.6, 0.6, 0.6)
    }

    /// Style du champ de texte lorsqu'il est désactivé.
    fn disabled(&self, _: &Self::Style) -> Appearance { //Mettre tout en gris
        text_input::Appearance {
            background: Background::Color(iced::Color::from_rgb(0.9, 0.9, 0.9)),
            border: Border::with_radius(5),
            icon_color: iced::Color::from_rgb(1., 1., 1.),
        }
    }
}