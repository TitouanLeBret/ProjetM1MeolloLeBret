//Page sur validite RSA Chiffrement
use iced::theme::{Theme};
use iced::widget::{button, container, text, Button, Column, Container, Row, Text, TextInput, text_input};
use iced:: { Alignment, Sandbox, Settings, Element, Background, Shadow, Vector, Border, Padding,Length};
use iced::alignment::{Horizontal, Vertical};


use super::gui;


/* 
#[derive(Default)]
struct ValidRsaChifPage {
    n_value: String,
    p_value: String,
    q_value: String,
    e_value: String,
    d_value: String,
    check_button: button::State,
    n_input: text_input::State,
    p_input: text_input::State,
    q_input: text_input::State,
    e_input: text_input::State,
    d_input: text_input::State,

}*/

//remplacer gui::Message par ValidRsaChifPage
pub fn check_enc_page_fn() -> Container<'static, gui::Message> {
    let n ="" ; let p ="" ; let q ="" ; let e ="" ; let d ="" ;
    let wrapper = Column::new()
        .push(text("Validité Chiffrement RSA").size(64))
        .push(input_field("N : ", n))
        .push(input_field("e : ", e))
        .push(input_field("p : ", p))
        .push(input_field("q : ", q))
        .push(input_field("d : ", d));
        //.push(chek_button);

    container(wrapper)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
}


//A mettre dans component : 
fn input_field( _placeholder:&str,_value : &str,) -> TextInput<'static, gui::Message> {
    TextInput::new(_placeholder,_value)
        .width(Length::Fixed(500.0))
        .padding(Padding::from(10))
        .line_height(text::LineHeight::Relative(1.75))
}