Readme du projet WEB : 

Réflexion sur les différents langages a utiliser our le projet WEB (gestion connexion, base de donnée, paiements etc..)
		-Django (Backend)
			https://www.youtube.com/watch?v=Bn0k9DDYBZM
		-MySQL ou PostgresSQL (BDD)
		-OAuth2 / OpenID Connect (Auth google, Facebook, et STRAVA)
		-JSON Web tokens (Authentification par Token)
		-Stripe (Paiements) et  Paypal  et/ou Mollie
		-React, Vue.js ou Angulaire (Frontend)
		-OVH, AWS … Pour hébergement
		-OWASP ZAP (tester failles pendant dev)
		-VOIR cloudinary pour gestion image et vidéo
		-VOIR OpenStreetMap (carte comme google maps) —> Avec Leaflet
		-VOIR OSRM (pour GPX et pt intérêt (ravit, contrôle))
		-VOIR Komoot API (carte GPX pour le tracé)
		-Stack technique pertinant : 
					Leaflet pour la carte (ou mambos)
					OpenrouteService pour GPX 
					Leaflet avec plugins d’altitud pour afficher l’altitude

Idée maquette WEB : 
        https://www.figma.com/design/amrZdVVDUFribXdWAmQT8O/Untitled?node-id=0-1&node-type=canvas&t=X3b16CzSb3xfNHJF-0


Pour lancer : 
	Pour créer env virtuel : 
		python -m venv .env
	Pour activer env virtel (windows) : 
		.\.env\Scripts\activate
		source .env/bin/activate (macos et linux)
	Pour désactiver env virtuel : 
		deactivate

	Vérifier que c'est bien l'env virutel : 
		Windows : py -c "import sys; print(sys.executable)" et voir si le chemin est bien dans DocBlog
		Linux : which python

	pour installer les dépendances : 
		Dans DocBlog : pip install -r requirements.txt


	Django : 
		Lancer serveur de production : cd src , python manage.py runserver




Source envoi mail verification: 
https://medium.com/@dszafranski/email-register-confirmation-django-django-allauth-299c4427726f


Pour amélioration Rapport :
On pourrait envoyer un mail après l'inscription a aune course, comme ça quand inscriptions
faites sans etre connecté, on envoie un mail a une adresse que l'user inscrit et celui-ci peut
lier cette inscription a un compte s'il veut en cliquant sur le lien et se connectant a son compte
