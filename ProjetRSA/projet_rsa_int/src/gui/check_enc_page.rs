//Page sur validite RSA Chiffrement
use iced::theme::{Theme};
use iced::widget::{button, container, text, text_input, Button, Checkbox, Column, Container, Row, Text, TextInput};
use iced:: { Alignment, Sandbox, Settings, Element, Background, Shadow, Vector, Border, Padding,Length};
use iced::alignment::{Horizontal, Vertical};
use iced_core::text::Paragraph;
use iced::Color;

use super::gui;
use crate::rsa::check_enc;
use super::components::MyTextInput;
use crate::rsa::check_enc::TestStatus; //Pour utiliser directemet TestStatus et pas check_enc::TestStatus

use crate::rsa::check_enc::all_status_to_false; //Pour remettre tous les status à false
use crate::rsa::check_enc::ALL_TEST_STATUS; //Pour utiliser la lsite de check_enc.rs

/// Gère l'état de la page "Validité Chiffrement RSA".
/// Cette structure contient les valeurs de clé publique et privée, 
/// ainsi qu'un bouton pour lancer les vérifications et une liste de tests.
#[derive(Default,Debug,Clone,PartialEq,Eq)]
pub struct ValidRsaChifPage {
    /// Valeur de N (clé publique).
    n_value: String,
    /// Valeur de p (clé privée, premier facteur premier).
    p_value: String,
    /// Valeur de q (clé privée, second facteur premier).
    q_value: String,
    /// Valeur de e (exposant public).
    e_value: String,
    /// Valeur de d (exposant privé).
    d_value: String,
    /// État du bouton pour vérifier la validité des clés.
    check_button: button::State,
    //La page de validation se sert de la liste de tests directement présente dans check_enc.rs, étant donnée que celle-ci est publique, nous n'avons pas besoin de garder une référence

    //Liste des messages d'erreur
    error_messages: Vec<String>,
}


/// Crée une nouvelle instance de `ValidRsaChifPage` avec des valeurs initiales vides
/// et un test de sécurité par défaut.
impl ValidRsaChifPage {
    pub fn new() -> Self {
        Self {
            n_value: String::new(),
            p_value: String::new(),
            q_value: String::new(),
            e_value: String::new(),
            d_value: String::new(),
            check_button: button::State::new(),
            error_messages: Vec::new(),
        }
    }



    /// Met à jour les valeurs de clé publique et privée.
    ///
    /// # Arguments
    ///
    /// * `n_val` - Nouvelle valeur pour N.
    /// * `p_val` - Nouvelle valeur pour p.
    /// * `q_val` - Nouvelle valeur pour q.
    /// * `e_val` - Nouvelle valeur pour e.
    /// * `d_val` - Nouvelle valeur pour d.
    pub fn update(&mut self ,n_val: String,p_val: String ,q_val: String ,e_val: String ,d_val: String) {
        self.n_value = n_val.clone();
        self.p_value = p_val.clone();
        self.q_value = q_val.clone();
        self.e_value = e_val.clone();
        self.d_value = d_val.clone();
    }


    /// Génère la vue de la page en affichant les champs de saisie, les boutons, et les résultats des tests.
    /// Organisation : 6 gros élements : 
    /// -Titre
    /// -Boutton génération de valeurs
    /// -Section avec champs pour clé publique
    /// -Section avec champs pour clé privée
    /// -Boutton de validation
    /// -Section pour les résultats des tests
    pub fn view(&self) -> Element<gui::Message> {
        let title = Text::new("Validité Chiffrement RSA")
            .size(48)
            .horizontal_alignment(Horizontal::Center)
            .style(Color::from_rgb(0.2, 0.2, 0.6));

        //Boutton pour générer une nouvelle clé RSA valide
        let new_values_button = button(text("Générer une clé RSA valide "))
            .padding(10)
            .on_press(gui::Message::NewValuesRsaEnc);

        //Section pour la partie de la clé publique
        let pub_key_section = Column::new()
            .spacing(10)
            .push(Text::new("Clé publique :"))
            .push(Row::new()// N : text field et e : text field
                .spacing(15)
                .push(Text::new("N :"))
                .push(MyTextInput::new("N", &self.n_value).width(Length::Fill).on_input(
                    | n_value | {
                        //Réinitialise tous les tests a False, car une case a était modifiée
                        all_status_to_false();  

                        gui::Message::FieldChangedRsaChiff(
                            n_value, 
                            self.p_value.clone(),
                            self.q_value.clone(),
                            self.e_value.clone(),
                            self.d_value.clone(),
                            )
                        
                        }
                    ))
                .push(Text::new("e :"))
                .push(MyTextInput::new("e", &self.e_value).width(Length::Fill).on_input(
                    | e_value | {
                        //Réinitialise tous les tests a False, car une case a était modifiée
                        all_status_to_false();
                        
                        gui::Message::FieldChangedRsaChiff(
                            self.n_value.clone(), 
                            self.p_value.clone(),
                            self.q_value.clone(),
                            e_value,
                            self.d_value.clone(),
                            )
                        }
                    ))
            );

        //Section pour la partie de la clé privée
        let priv_key_section = Column::new()
            .spacing(10)
            .push(Text::new("Clé privée :"))
            .push(Row::new()
                .spacing(15)
                .push(Text::new("p :"))
                .push(MyTextInput::new("p", &self.p_value).width(Length::Fill).on_input(
                    | p_value | {
                        //Réinitialise tous les tests a False, car une case a était modifiée
                        all_status_to_false();

                        gui::Message::FieldChangedRsaChiff(
                            self.n_value.clone(), 
                            p_value,
                            self.q_value.clone(),
                            self.e_value.clone(),
                            self.d_value.clone(),
                            )
                        }
                    ))
                .push(Text::new("q :"))
                .push(MyTextInput::new("q", &self.q_value).width(Length::Fill).on_input(
                    | q_value | {
                        //Réinitialise tous les tests a False, car une case a était modifiée
                        all_status_to_false();

                        gui::Message::FieldChangedRsaChiff(
                            self.n_value.clone(), 
                            self.p_value.clone(),
                            q_value,
                            self.e_value.clone(),
                            self.d_value.clone(),
                            )
                        }
                    ))
                .push(Text::new("d :"))
                .push(MyTextInput::new("d", &self.d_value).width(Length::Fill).on_input(
                    | d_value | {
                        //Réinitialise tous les tests a False, car une case a était modifiée
                        all_status_to_false();

                        gui::Message::FieldChangedRsaChiff(
                            self.n_value.clone(), 
                            self.p_value.clone(),
                            self.q_value.clone(),
                            self.e_value.clone(),
                            d_value,
                            )
                        }
                    ))
            );

        //Boutton pour vérifier la validité des clés RSA
        let check_button = button(text("Vérifier la validité"))
            .padding(10)
            .on_press(gui::Message::CheckButtonPressedRsaChiff);

        // Section des tests de sécurité avec les cases validées ou non
        let test_results = ALL_TEST_STATUS.lock().unwrap().iter()
            .fold(Column::new()
                .spacing(10), |column: Column<'_, gui::Message>, test| { column
                    .push(
                        Row::new()
                            .spacing(10)
                            .push(Text::new(test.name))
                            .push(
                                Checkbox::new(
                                    "",
                                    test.is_valid,
                                )
                                //.style(if test.is_valid { ValidStyle::Checked } else { ValidStyle::Unchecked })
                            )
                    )
                }
            );
        
        // Section des messages d'erreur
        let error_message_section: Column<'_, gui::Message> = self.error_messages.iter().fold(
            Column::new().spacing(5),
            |column, error| {
                column.push(Text::new(error.clone()).style(Color::from_rgb(0.8, 0.2, 0.2)))
            },
        );


        let wrapper = Column::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(title)
            .push(new_values_button)
            .push(pub_key_section)
            .push(priv_key_section)
            .push(check_button)
            .push(test_results)
            .push(error_message_section);

        container(wrapper)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_x()
            .center_y() // Fond standard
            .into()
    }

    //Getter et setter :

    /// Retourne l'état actuel des tests de sécurité. 
    pub fn get_tests_status(&self) -> Vec<TestStatus> {
        ALL_TEST_STATUS.lock().unwrap().clone()
    }


    /// Méthode de test : Affiche dans la console les valeurs saisies, ainsi que le produit p*q.
    pub fn display_values(&self) {
        let p : num_bigint::BigUint = self.p_value.parse().expect("Echec conversion");
        let q : num_bigint::BigUint = self.q_value.parse().expect("Echec conversion");
        println!("Values are: N:{}\n, E:{}\n, P:{}\n, Q:{}\n, D:{}\n, p*q = {}\n",self.n_value.clone(), self.e_value.clone(),self.p_value.clone(),self.q_value.clone(),self.d_value.clone(),p*q);
    }


    fn add_error_message(&mut self, msg: &str) {
        self.error_messages.push(msg.to_string());
    }

    pub fn remove_all_error_message(&mut self){
        self.error_messages.clear();
    }

    /// Vérifie la validité de la clé RSA en exécutant tous les tests de sécurité.
    /// Met à jour les résultats des tests dans l'état de la page.
    pub fn check_values(&mut self){
        //Test pour voir si toutes les cases sont bien convertibles en int
        self.remove_all_error_message();
        match validate_inputs(&self.n_value.clone(), &self.e_value.clone(), &self.p_value.clone(), &self.q_value.clone(), &self.d_value.clone()) {
            Ok(_) => {
                check_enc::calc_all_security_tests_status(self.n_value.clone(), self.e_value.clone(), self.p_value.clone(), self.q_value.clone(), self.d_value.clone());
            }
            Err(errors) => {
                // Affiche une popup ou un message d'erreur
                let error_message = format!(
                    "Attention, les erreurs suivantes ont été détectées :\n{}",
                    errors.join("\n")
                );
                self.add_error_message(&error_message);
            }
        }
    }

}


use std::str::FromStr;
/// Fonction de validation des entré (vérif que ce sont bien des entiers valide)
fn validate_inputs(n_value: &str, e_value: &str, p_value: &str, q_value: &str, d_value: &str) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    // Essayer de convertir chaque valeur
    if num_bigint::BigUint::from_str(n_value).is_err() {
        errors.push("N n'est pas un entier valide.".to_string());
    }
    if num_bigint::BigUint::from_str(e_value).is_err() {
        errors.push("E n'est pas un entier valide.".to_string());
    }
    if num_bigint::BigUint::from_str(p_value).is_err() || num_bigint::BigUint::from_str(p_value).unwrap() <= num_bigint::BigUint::from(1u32) {
        errors.push("P n'est pas un entier valide (doit etre > 1).".to_string());
    }
    if num_bigint::BigUint::from_str(q_value).is_err() || num_bigint::BigUint::from_str(q_value).unwrap() <= num_bigint::BigUint::from(1u32) {
        errors.push("Q n'est pas un entier valide (doit etre > 1).".to_string());
    }
    if num_bigint::BigUint::from_str(d_value).is_err() {
        errors.push("D n'est pas un entier valide.".to_string());
    }

    if errors.is_empty() {
        Ok(()) // Pas d'erreurs
    } else {
        Err(errors) // Renvoie les erreurs
    }
}