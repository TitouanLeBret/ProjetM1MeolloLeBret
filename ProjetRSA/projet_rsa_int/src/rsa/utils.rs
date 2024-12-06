//Fichier comportants toutes les fonctions vouées a ressevir dans plusieurs fichiers
use rsa::BigUint as RsabgBigUint;
use num_traits::Zero;

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



pub fn pgcd(a:&RsabgBigUint,b:&RsabgBigUint) -> RsabgBigUint {
    if b.is_zero(){
        //cas de base :
        return a.clone();
    }

    let _q = a / b; //quotient
    let r = a % b; //reste

    let pgcd = pgcd(b,&r);

    return pgcd;
}