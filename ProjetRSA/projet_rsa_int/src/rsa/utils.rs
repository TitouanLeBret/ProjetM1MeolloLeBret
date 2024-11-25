//Fichier comportants toutes les fonctions vouées a ressevir dans plusieurs fichiers
use rsa::BigUint as RsabgBigUint;
use num_traits::{Zero, One};
use std::ops::{Div, Rem};

pub fn pgcd(a:&RsabgBigUint,b:&RsabgBigUint) -> RsabgBigUint {
    if b.is_zero(){
        //cas de base :
        return a.clone();
    }

    let q = a / b; //quotient
    let r = a % b; //reste

    let pgcd = pgcd(b,&r);

    return pgcd;
}