// Rust UI - Iced

// Modules ... 
use iced::theme::{Theme};
use iced::widget::{button, container, text, Button, Column, TextInput,Text};
use iced:: { Alignment, Sandbox, Settings, Element, Background, Shadow, Vector, Border, Padding,Length};
use iced::alignment::{Horizontal, Vertical};

//Nos propres modules 
use super::check_enc_page;
use super::components;
use super::home_page;







pub struct App{
    current_page: Page, // Page 1,2 ,3 etc 
    home_page : home_page::HomePage,
    valid_rsa_chif_page: check_enc_page::ValidRsaChifPage,
    valid_rsa_sign_page : home_page::HomePage,
    theme: Theme, // Noir ou blanc
}

impl Sandbox for App{
    type Message = Message;

    fn new() -> Self {
        Self {
            current_page: Page::Home, // Page 1,2 ,3 etc 
            home_page : home_page::HomePage::new(),
            valid_rsa_chif_page: check_enc_page::ValidRsaChifPage::new(),
            valid_rsa_sign_page : home_page::HomePage::new(),
            theme: Theme::Dark, // Drak theme
        }
    }

    //Définir la méthode update
    fn update(&mut self, message: Message) {
        match message {

            Message::Router(page)=> {
                self.current_page = page;
            }
//Méthode pour page RSA Chiffrement :

            Message::FieldChangedRsaChiff(n_val,p_val ,q_val ,e_val ,d_val ) => {
                self.valid_rsa_chif_page.update(n_val,p_val ,q_val ,e_val ,d_val );
            }

            Message::CheckButtonPressedRsaChiff =>{
                println!("Check button pressed");
                self.valid_rsa_chif_page.display_values();
                self.valid_rsa_chif_page.check_values();
            }




//Méthode pour changer le Thème :

            Message::ToggleTheme => {
                self.theme = if self.theme == Theme::Light {
                    Theme::Dark
                }else {
                    Theme::Light
                }
            }
            
        }
    }


    //Méthode view -> c'est ou l'UI va chercher la page
    fn view(&self) -> Element<Message> {
        match self.current_page {
            Page::Home => self.home_page.view(),
            Page::ValiditeRSAChiffrement => self.valid_rsa_chif_page.view(),
            Page::ValiditeRSASignature => self.home_page.view(),
        }
    }



    //definir le titre de l'app
    fn title(&self) -> String {
        String::from("Projet Rust RSA")
    }

    fn theme(&self) ->Theme {
        self.theme.clone() //retourne une copie du theme
    }

}














//enum pour les pages, chaque variable dans Page va créer une nouvelle view/page
#[derive(Debug,Clone,PartialEq,Eq)]
pub enum Page {
    Home,
    ValiditeRSAChiffrement,
    ValiditeRSASignature,
    /*
    SecuriteRSAChiffrement,
    SecuriteRSASignature
    */
} // liste des pages



//callback events
#[derive(Debug,Clone)]
pub enum Message{
    Router(Page), // Change la page en fonction de la route indique
    //PageSpecific(PageMessage),
    //Méthode call back RSA Chiffrement
    FieldChangedRsaChiff(String, String, String, String, String),
    CheckButtonPressedRsaChiff,


    //'a est le lifetime de la ref emprunte sur ValidRsaChifPage, valable aussi longtemp que Message<'a>
    ToggleTheme, //light/dark


}


#[derive(Debug, Clone)]
pub enum PageMessage {
    // Messages spécifiques à chaque page
    FieldChanged(String),
    ButtonPressed,
}


pub trait PageContent {
    fn view(&self) -> Element<Message>;
    fn update(&mut self, message: PageMessage);
}


























//Style des champs d'entrées

fn input_field( _placeholder:&str,_value : &str,) -> TextInput<'static, Message>{
    TextInput::new(_placeholder,_value)
        .width(Length::Fixed(500.0))
        .padding(Padding::from(10))
        .line_height(text::LineHeight::Relative(1.75))
}

//Style du boutton

pub enum ButtonStyle {Standard, ThemeButton}

impl button::StyleSheet for ButtonStyle {
    type Style = Theme;

    //definir comportement actif 
    fn active(&self, theme : &Self::Style) -> button::Appearance{
        button::Appearance {
            background: Some(Background::Color(match self {
                Self::Standard => iced::Color::from_rgb(0.059,0.463,0.702),
                Self::ThemeButton => iced::Color::default(),
            })),
            border : match self {
                Self::Standard => Border::with_radius(5),
                Self::ThemeButton => Border::default(),
            },
            shadow: match self {
                Self::Standard => Shadow {
                    color : iced::Color::BLACK,
                    offset: Vector::new(0.0,4.0),
                    blur_radius:20.0,
                },
                Self::ThemeButton => Shadow::default(),
            },
            text_color : {
                if theme == &Theme::Light {
                     match self {
                        Self::Standard => iced::Color::WHITE,
                        Self::ThemeButton => iced::Color::BLACK,
                     }
                } else {
                     match self {
                        Self::Standard => iced::Color::BLACK,
                        Self::ThemeButton => iced::Color::WHITE,
                     }
                }
            },
            ..Default::default()
        }
    }
}

//Summit du boutton

fn submit_btn(name: &str, event:Message) -> Button<Message> {
    Button::new(
        text(name)
        .horizontal_alignment(Horizontal::Center)
        .vertical_alignment(Vertical::Center)
        .size(21)
    )
    .on_press(event)
    .width(Length::Fixed(500.0))
    .height(Length::Fixed(45.0))
    .style(iced::theme::Button::Custom(Box::new(ButtonStyle::Standard)))
}

//définir le style du conteneur 
struct ContainerStyle;

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