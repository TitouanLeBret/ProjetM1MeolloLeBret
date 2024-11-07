// Modules ... 
use iced::theme::{Theme};
use iced::widget::{button, container, text, Button, Column, Container, Row, Text, TextInput};
use iced:: { Alignment, Sandbox, Settings, Element, Background, Shadow, Vector, Border, Padding,Length};
use iced::alignment::{Horizontal, Vertical};


use super::gui;

//pied de page (a transformer en haut de page)
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

pub struct ContainerStyle;

impl container::StyleSheet for ContainerStyle {
    type Style = Theme;

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


pub struct myField {value : String, label:String}
