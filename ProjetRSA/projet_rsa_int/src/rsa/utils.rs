//Fichier comportants toutes les fonctions vouées a ressevir dans plusieurs fichiers
use rsa::BigUint as RsaBigUint;
use num_traits::Zero;
use num_bigint::{BigInt,BigUint};

/// Représente le statut d'un test de sécurité.
/// Chaque test a un nom (`name`) et un statut de validation (`is_valid`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TestStatus {
    pub name: &'static str,
    pub is_valid: bool,
}

//Remets tous les .is_valid a false (réinitialise les status)
pub fn all_status_to_false(list_test_status: &mut Vec<TestStatus>){
    let tests = list_test_status;
    for test in tests.iter_mut() {
        test.is_valid = false;
    }
}

//Mets le status a l'index "index" a la valeur new_status
pub fn update_test_status(list_test_status: &mut Vec<TestStatus>,index: usize, new_status: bool) {
    let tests = list_test_status;
    if let Some(test) = tests.get_mut(index) {
        test.is_valid = new_status;
    }
}



pub fn pgcd(a:&RsaBigUint,b:&RsaBigUint) -> RsaBigUint {
    if b.is_zero(){
        //cas de base :
        return a.clone();
    }

    let _q = a / b; //quotient
    let r = a % b; //reste

    let pgcd = pgcd(b,&r);

    return pgcd;
}

pub fn bezout(x: &BigInt, y: &BigInt) -> (BigInt, BigInt, BigInt) {
    let mut x = x.clone();
    let mut y = y.clone();
    let mut u0 = BigInt::from(1u8);
    let mut u1 = BigInt::from(0u8);
    let mut v0 = BigInt::from(0u8);
    let mut v1 = BigInt::from(1u8);

    while !y.is_zero() {
        let q = &x / &y;
        let r = &x % &y;
        x = y;
        y = r;

        let temp_u1 = u1.clone();
        u1 = &u0 - &q * &u1;
        u0 = temp_u1;

        let temp_v1 = v1.clone();
        v1 = &v0 - &q * &v1;
        v0 = temp_v1;
    }

    (x, u0, v0)
}

pub fn inverse(x:&RsaBigUint,n:&RsaBigUint) -> RsaBigUint {
    let pgcd = pgcd(x,n);
    if pgcd != RsaBigUint::from(1u8) {
        return RsaBigUint::from(0u8);
    }
    let x_bigint = BigInt::from(BigUint::from_bytes_be(&x.to_bytes_be()));
    let n_bigint =BigInt::from(BigUint::from_bytes_be(&n.to_bytes_be()));
    let (_, u, _) = bezout(&x_bigint, &n_bigint);
    let res = ((u.clone() % n_bigint.clone()) + n_bigint.clone()) % n_bigint.clone(); // ça fait u % n_bigint, mais ça s'assure que le résultats soit positif
    println!("{:?}", res);
    let (sign, bytes) = res.to_bytes_be();
    if sign == num_bigint::Sign::Minus {
        panic!("Cannot convert negative BigInt to RsaBigUint");
    }
    RsaBigUint::from_bytes_be(&bytes)
}