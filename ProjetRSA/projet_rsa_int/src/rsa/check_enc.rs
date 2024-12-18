//docs : https://docs.rs/rsa/latest/rsa/
use rsa::{RsaPrivateKey, RsaPublicKey };
use rsa::BigUint as RsaBigUint;
use rsa::traits:: {PrivateKeyParts,PublicKeyParts}; 
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::str::FromStr;
use num_primes::Verification;
use rsa::Pkcs1v15Encrypt; // C'est le padding Pkcs1
use super::utils::TestStatus;
use super::utils::update_test_status;
use super::utils::pgcd; 


// Définition d'un vecteur global Mutex contenant les statuts des tests RSA.
// Ce vecteur est mutable net peut être utilisé dans différetes pages, comme dans check_enc_page.rs.
pub static ALL_TEST_STATUS_VALID_RSA : Lazy<Mutex<Vec<TestStatus>>> = Lazy::new(||
    Mutex::new(vec![
    TestStatus {
        name: "Test de la taille de N (>= 2048 bits)",
        is_valid: false,
    },
    TestStatus {
        name: "Test de factorisation (N=p*q)",
        is_valid: false,
    },
    TestStatus {
        name: "Test de chiffrement / déchiffrement",
        is_valid: false,
    },
    TestStatus {
        name: "Test de sécurité sur e et d",
        is_valid: false,
    },
    TestStatus {
        name: "Test de sécurité complet",
        is_valid: false,
    },
    TestStatus {
        name: "Rajouter test dans ALL_TEST_STATUS",
        is_valid: false,
    },
    ])
);

// Fonction principale pour calculer tous les tests de sécurité RSA en prenant les valeurs N, e, p, q et d en entrée.
// Elle met à jour le statut des tests dans le vecteur global ALL_TEST_STATUS_VALID_RSA.
pub fn calc_all_security_tests_status(n_value : String , e_value: String, p_value: String, q_value: String , d_value: String) {
    //Return une énum avec les différents tests associés a leur validité ou non
    let n = RsaBigUint::from_str(&n_value).expect("Conversion échouée pour N ");
    let e = RsaBigUint::from_str(&e_value).expect("Conversion échouée pour e");
    let p = RsaBigUint::from_str(&p_value).expect("Conversion échouée pour p");
    let q = RsaBigUint::from_str(&q_value).expect("Conversion échouée pour q");
    let d = RsaBigUint::from_str(&d_value).expect("Conversion échouée pour d");
    let priv_key : RsaPrivateKey = RsaPrivateKey::from_components(n.clone(), e.clone(), d.clone(),vec![p.clone(), q.clone()]).expect("Conversion échouée");
    let pub_key : RsaPublicKey = RsaPublicKey::from(&priv_key);
    let validation_bits_pub_key =  bits_pub_key(&n) ;
    let validation_facto = is_valid_factorisation(&n, &p, &q) ;
    let validation_enc_dec =  is_valid_encryption_decryption( &pub_key, &priv_key) ;
    let validation_e_d = are_valide_e_d(&pub_key,&priv_key);
    let validation_complete = validation_bits_pub_key && validation_facto && validation_enc_dec && validation_e_d;
    update_test_status(&mut ALL_TEST_STATUS_VALID_RSA.lock().unwrap(),0,validation_bits_pub_key);
    update_test_status(&mut ALL_TEST_STATUS_VALID_RSA.lock().unwrap(),1,validation_facto);
    update_test_status(&mut ALL_TEST_STATUS_VALID_RSA.lock().unwrap(),2,validation_enc_dec);
    update_test_status(&mut ALL_TEST_STATUS_VALID_RSA.lock().unwrap(),3,validation_e_d);
    update_test_status(&mut ALL_TEST_STATUS_VALID_RSA.lock().unwrap(),4,validation_complete);
}


//Toutes les fonctions utilisés dans all_security_tests :
fn bits_pub_key(n : &RsaBigUint) -> bool {
    let n_bits = n.bits(); // Renvoie le nombre de bits de n
    n_bits >= 2048 //Vrai si >=2048 faut sinon
}


fn primalite(p : &RsaBigUint) -> bool {
    //2 lignes du dessous pour converitr RsaBigUint en num_primes::BigUint
    let bytes_p = p.to_bytes_be();
    let p_prime = num_primes::BigUint::from_bytes_be(&bytes_p);
    Verification::is_prime(&p_prime)
}

//Vérifie que p et q sont premiers
fn are_prime(p : &RsaBigUint, q : &RsaBigUint) -> bool {
    primalite(p) && primalite(q)
}


//Verifie que p*q =n et que p et q sont premiers
fn is_valid_factorisation(n : &RsaBigUint, p : &RsaBigUint, q : &RsaBigUint) -> bool {
    let n_calc = p *q; // Calcul du produit des facteurs premiers
    n.eq(&n_calc) && are_prime(p, q)
}


//Voir : https://docs.rs/rsa/latest/rsa/index.html
//Fonction qui verifie la validité du chiffrement/dechiffrement
fn is_valid_encryption_decryption(pub_key : &RsaPublicKey, priv_key : &RsaPrivateKey) -> bool {
    let data = b"Hello world! C'est  le test de chiffrement/dechiffrement";
    let mut rng = rand::thread_rng();
    let enc_data = pub_key.encrypt(&mut rng,Pkcs1v15Encrypt, &data[..]).unwrap_or_else(|_| vec![]); // si on ne peut pas unwrap on renvoie un vecteur vide

    let dec_data = priv_key.decrypt(Pkcs1v15Encrypt, &enc_data).unwrap_or_else(|_| vec![]);// si on ne peut pas unwrap on renvoie un vecteur vide
    data == &dec_data[..] // On retourne vrai si c'est égale, faux sinon

}


//Verifie que e et d sont bien valide
//pgcd e et phi(n) = 1
//e*d % phi(n) = 1
fn are_valide_e_d(pub_key : &RsaPublicKey, priv_key : &RsaPrivateKey) -> bool {
    // Récupérer les valeurs de la clé publique et privée
    let e = pub_key.e(); // Exposant public
    let d = priv_key.d(); // Exposant privé
    let _n = pub_key.n(); // Modulus (N)
    let p = priv_key.primes()[0].clone(); // Premier facteur de N
    let q = priv_key.primes()[1].clone(); // Deuxième facteur de N

    // Calculer φ(N) = (p - 1) * (q - 1)
    let phi_n = (p - RsaBigUint::from(1u8)) * (q - RsaBigUint::from(1u8));

    // Vérifier que e et φ(N) sont premiers entre eux grâce aui pgcd
    if pgcd(&e,&phi_n) != RsaBigUint::from(1u8) {
        return false; // e n'est pas valide
    };

    // Vérifier que (e * d) % φ(N) == 1
    if (e * d) % &phi_n != RsaBigUint::from(1u8) {
        return false; // d n'est pas valide
    };

    true
}
