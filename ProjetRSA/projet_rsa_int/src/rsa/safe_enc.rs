use super::utils::TestStatus;
use super::utils::update_test_status;
use once_cell::sync::Lazy;
use std::str::FromStr;
use crate::gui::safe_enc_page::SafeRsaChifPage;
use rsa::{BigUint as RsaBigUint, RsaPublicKey};
use std::sync::Mutex;
use num_primes::Verification;

// Définition d'une variable globale protégée par Mutex (pour la sécurité des accès concurrents)
pub static ALL_TEST_STATUS_SECU_RSA : Lazy<Mutex<Vec<TestStatus>>> = Lazy::new(||
    Mutex::new(vec![
    TestStatus {
        name: "Aucune faille detectée",
        is_valid: false,
    },
    TestStatus {
        name: "Est ce que N = p² ? ",
        is_valid: false,
    },
    TestStatus {
        name : "Est ce que N est factorisable car trop petit ? ",
        is_valid: false,
    },
    TestStatus {
        name : "Est ce que N est premiers (et on peut donc déchiffrer le ct)? ",
        is_valid: false,
    },
    TestStatus {
        name : "Autre test à implémenter dans le futur...",
        is_valid: false,
    },
    ])
);

// Fonction qui prend en paramètres phi_n, n, e, ct et une page d'affichage, puis effectue le déchiffrement
// Elle affiche également le message déchiffré sur la page et vérifie si le ciphertext est correct
fn find_message(phi_n : &RsaBigUint, n: &RsaBigUint, e : &RsaBigUint, ct:&RsaBigUint,safe_enc_page: &mut SafeRsaChifPage) -> RsaBigUint {
    let d = super::utils::inverse(e, phi_n);
    let message = ct.modpow(&d,n);
    safe_enc_page.display_message(&String::from_utf8_lossy(&message.to_bytes_be()));
    let ct_test = message.modpow(e,n);
    return ct_test
}

//Test si n est un carré parfait (n = p²)
fn test_n_facteur_carre(n: &RsaBigUint,e:&RsaBigUint,ct : &RsaBigUint,safe_enc_page: &mut SafeRsaChifPage) -> bool{
    //Vérifie si n est un carré
    let p: RsaBigUint = n.sqrt();
    if &p * &p == *n {
        let phi_n = &p*(&p-RsaBigUint::from(1u8));
        if super::utils::pgcd(e,&phi_n) == RsaBigUint::from(1u8) { // Pour etre sur que l'inverse existe
            return &find_message(&phi_n,n,e,ct,safe_enc_page)==ct;
        }else {
            return false;
        }
    }
    false
}


//test de factorisation de n s'il est trop petit
//Implémentation 0.1, dans le futur il faudrait un algortithme de factorisation ou un requete au site factordb
fn test_is_factorisable_too_small(n : &RsaBigUint,safe_enc_page: &mut SafeRsaChifPage) -> bool {
    if n.bits() < 2048{
        //Implémenter un algorithme de factorisation, ou se servir de "reqwest", pour envoyer une requete a factordb
        //Pour cela il faut apprendre a bien utiliser async, et voir si c'est possible.
        //Dans cette version de l'application (rendu final), on regarde juste si N est trop petit, auquel cas l'utilisateur
        //Peut aller voir sur factordb
        safe_enc_page.display_message(&String::from("N est factorisable car il est trop petit, par manque de temps nous n'avons pas implémenté cette fonctionnalité, mais n'hésitez pas a vous rendre sur factordb.com"));
        return true;
    }
    false
}


//Vérifie que n est premier, si c'est pas le cas, la factorisation est simple 
fn n_is_prime(n: &RsaBigUint,e:&RsaBigUint,ct : &RsaBigUint,safe_enc_page: &mut SafeRsaChifPage) -> bool {
    let bytes_n = n.to_bytes_be();
    let n_prime = num_primes::BigUint::from_bytes_be(&bytes_n);
    if Verification::is_prime(&n_prime){
        let phi_n = n - RsaBigUint::from(1u8);
        let ct_test = find_message(&phi_n,&n,&e,&ct,safe_enc_page);
        return &ct_test==ct
    }
    false
}

// Fonction qui crée une clé publique à partir de n et e
fn create_public_key(
    n: &RsaBigUint,
    e: &RsaBigUint,
) -> Option<RsaPublicKey> {
    match RsaPublicKey::new(n.clone(), e.clone()) {
        Ok(pub_key) => Some(pub_key),//clé publique si elle est créee   
        Err(_) => None//None si erreur
    }
}

// Vérifie si tous les tests ont échoué
// si c'est le cas, valide la case "Aucune faille detectée"
fn all_test(list_test_status: &mut Vec<TestStatus>) -> bool {
    let mut all_test_are_false = false;
    for status in list_test_status.iter().skip(1) {
        all_test_are_false = all_test_are_false || status.is_valid;
    }
    return !all_test_are_false
}


// Fonction principale qui calcule tous les statuts de sécurité et met à jour l'interface en fonction des résultats
pub fn calc_all_safety_status(safe_enc_page: &mut SafeRsaChifPage, n_value : String , e_value: String, ct_value: String) {
    //Return une énum avec les différents tests associés a leur validité ou non
    let n = RsaBigUint::from_str(&n_value).expect("Conversion échouée");
    let e = RsaBigUint::from_str(&e_value).expect("Conversion échouée");
    let ct = RsaBigUint::from_str(&ct_value).expect("Conversion échouée");
    let _p = RsaBigUint::from_str(&ct_value).expect("Conversion échouée");
    let _pub_key : RsaPublicKey = match create_public_key(&n,&e){
        Some(pub_key) => pub_key,
        None => {
            safe_enc_page.add_error_message("Il y a eu une erreur lors de la création de la clé publique");
            return;
          }  //si erreur : on met un message d'erreur et on sort complétement de la fonction calc_all_safety_status
    };
    update_test_status(&mut ALL_TEST_STATUS_SECU_RSA.lock().unwrap(), 1, test_n_facteur_carre(&n,&e, &ct,safe_enc_page));
    update_test_status(&mut ALL_TEST_STATUS_SECU_RSA.lock().unwrap(), 2, test_is_factorisable_too_small(&n,safe_enc_page));
    update_test_status(&mut ALL_TEST_STATUS_SECU_RSA.lock().unwrap(), 3, n_is_prime(&n,&e, &ct,safe_enc_page));
    let all_test_false = all_test(&mut ALL_TEST_STATUS_SECU_RSA.lock().unwrap());
    update_test_status(&mut ALL_TEST_STATUS_SECU_RSA.lock().unwrap(), 0,all_test_false);
} 