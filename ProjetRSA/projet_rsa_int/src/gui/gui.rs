// Rust UI - Iced

// Modules ... 
use iced::theme::Theme;
use iced::widget::button;
use iced:: { Sandbox, Element, Background, Shadow, Vector, Border};

//Nos propres modules 
use super::check_enc_page;
use super::safe_enc_page;
use super::home_page;

use crate::rsa::keygen::generate_rsa_private_key;
use crate::rsa::keygen::generate_rsa_public_key;






pub struct App{
    current_page: Page, // Page 1,2 ,3 etc 
    home_page : home_page::HomePage,
    valid_rsa_chif_page: check_enc_page::ValidRsaChifPage,
    secu_rsa_chif_page : safe_enc_page::SafeRsaChifPage,
    theme: Theme, // Noir ou blanc
}

impl Sandbox for App{
    type Message = Message;

    fn new() -> Self {
        Self {
            current_page: Page::Home, // Page 1,2 ,3 etc 
            home_page : home_page::HomePage::new(),
            valid_rsa_chif_page: check_enc_page::ValidRsaChifPage::new(),
            secu_rsa_chif_page : safe_enc_page::SafeRsaChifPage::new(),
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

            Message::FieldChangedRsaChiffSecu(n_val,e_val ,ct_val ) => {
                self.secu_rsa_chif_page.update(n_val,e_val,ct_val);
            }

            Message::CheckButtonPressedRsaChiff =>{
                self.valid_rsa_chif_page.check_values();
            }

            Message::CheckButtonPressedRsaChiffSecu =>{
                self.secu_rsa_chif_page.remove_display_message();
                self.secu_rsa_chif_page.check_values();
            }

            Message::NewValuesRsaEnc => {
                let key = generate_rsa_private_key(2048);
                self.valid_rsa_chif_page.reset_status(); // Fonction qui va appeler all_status_to_false, mais faites pour ne pas avoir a passer ALL_TEST_STATUS_VALID_RSA ici 
                self.valid_rsa_chif_page.remove_all_error_message(); // On mets de nouvelles valeurs donc on remet les status de tests a false
                self.valid_rsa_chif_page.update(key[0].to_string(),key[2].to_string(),key[3].to_string(),key[1].to_string(),key[4].to_string());
            }

            Message::NewValuesRsaEncSecu => {
                let key = generate_rsa_public_key(2048); // Génere une clé publique valide et un ct valide
                self.secu_rsa_chif_page.reset_status(); // Fonction qui va appeler all_status_to_false, mais faites pour ne pas avoir a passer ALL_TEST_STATUS_VALID_RSA ici 
                self.secu_rsa_chif_page.remove_all_error_message(); // On mets de nouvelles valeurs donc on remet les status de tests a false
                self.secu_rsa_chif_page.remove_display_message();
                self.secu_rsa_chif_page.update(key[0].to_string(),key[1].to_string(),key[2].to_string());
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
            Page::SecuriteRsaChiffrement => self.secu_rsa_chif_page.view(),
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
    SecuriteRsaChiffrement,
} // liste des pages



//callback events
#[derive(Debug,Clone)]
pub enum Message{
    Router(Page), // Change la page en fonction de la route indique
    //PageSpecific(PageMessage),
    //Méthode call back RSA Chiffrement
    FieldChangedRsaChiff(String, String, String, String, String),
    FieldChangedRsaChiffSecu(String,String,String),
    CheckButtonPressedRsaChiff,
    CheckButtonPressedRsaChiffSecu,
    NewValuesRsaEnc,
    NewValuesRsaEncSecu,

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

