use super::utils::TestStatus;
use super::utils::update_test_status;
use once_cell::sync::Lazy;
use std::str::FromStr;

use crate::gui::safe_enc_page::SafeRsaChifPage;
use rsa::{BigUint as RsaBigUint, RsaPublicKey};
use std::sync::Mutex;
//Ce vecteur a un Mutex, pour pouvoir etre une variable globale mutable,
//On s'en sert dans check_enc_page.rs, mais on pourrais récréer une autre page qui s'en sert sans problème
pub static ALL_TEST_STATUS_SECU_RSA : Lazy<Mutex<Vec<TestStatus>>> = Lazy::new(||
    Mutex::new(vec![
    TestStatus {
        name: "Test de sécurité sur RSA",
        is_valid: false,
    },
    ])
);

fn test_qui_fait_rien() -> bool{
    return true
}

//Fonction pour créer la clé
fn create_public_key(
    n: RsaBigUint,
    e: RsaBigUint,
) -> Option<RsaPublicKey> {
    match RsaPublicKey::new(n, e) {
        Ok(pub_key) => Some(pub_key),//clé publique si elle est créee   
        Err(_) => None//None si erreur
    }
}

pub fn calc_all_safety_status(safe_enc_page: &mut SafeRsaChifPage, n_value : String , e_value: String, ct_value: String) {
    //Return une énum avec les différents tests associés a leur validité ou non
    let n = RsaBigUint::from_str(&n_value).expect("Conversion échouée");
    let e = RsaBigUint::from_str(&e_value).expect("Conversion échouée");
    let _p = RsaBigUint::from_str(&ct_value).expect("Conversion échouée");
    let _pub_key : RsaPublicKey = match create_public_key(n,e){
        Some(pub_key) => pub_key,
        None => {
            safe_enc_page.add_error_message("Il y a eu une erreur lors de la création de la clé publique");
            return;
          }  //si erreur : on met un message d'erreur et on sort complétement de la fonction calc_all_safety_status
    };
    update_test_status(&mut ALL_TEST_STATUS_SECU_RSA.lock().unwrap(),0,test_qui_fait_rien());
} 