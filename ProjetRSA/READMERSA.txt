Readme du projet RSA :

Pourquoi Rust : conférence + article
		Conf : https://www.youtube.com/watch?v=I2iYNZm_pqg&list=PLrT8DrHsxZTiiAj96QukmAdedfRMsIPN5&index=4
		Article : https://www.editions-eni.fr/blog/5-bonnes-raisons-dutiliser-le-langage-rust/
	
	Test Rust interface graphique : https://www.youtube.com/watch?v=72PyU1EIGY8
	Tuto rust fait : Apprendre le RUST partie #1,2,3 FR
					https://www.youtube.com/watch?v=mZasv3__A9k&list=PLrT8DrHsxZTiiAj96QukmAdedfRMsIPN5
					https://www.youtube.com/watch?v=wgjw5lGv-EI&list=PLrT8DrHsxZTiiAj96QukmAdedfRMsIPN5&index=2
					https://www.youtube.com/watch?v=3kBk3sjREOM&list=PLrT8DrHsxZTiiAj96QukmAdedfRMsIPN5&index=3

A implémenter : 
	Chiffrement RSA :
		Vérification si porte dérobé ? (Peut etre pas possible a faire ? ) -> TP7
		Vérification qu'on a bien N=p*q et pas juste N premier
		Vérification N= p*q avec p différent de q et pas N=p²
		Vérifier que N n'es pas un facteur de trop nombreux nombre premier (p1*p2*....*p30)
		Test factorisation classique (factordb.com)
		Test liste_marin 


	Signature RSA : 
		Attaque de Bellcore ? (TP6) -> sur RSA-CRT


4 page différentes : ValiditeChiffrementRSA / ValiditeSignatureRSA / SecuriteChiffrementRSA / SecuriteSignatureRSA
Pour l'instant dans le dossier fonctions, début de ValiditeChiffrement, et dans icedTuto, debut de l'interface graphique mais qui ne fais rien


A faire : 
	Implémenter tous les tests de check validation RSA, il reste test E, D, et peut etre d'autres...