Pour base de données :
    on créer une App : python .\manage.py startapp inscription_course
    dans models.py on créer des classes qui représente nos modeles (lers infos qu'on veut dans la DB)
    on ajoute cette app dans le settings.py du site
    on makemigrations : python .\manage.py makemigrations inscription_course
    on applique les migrations :  python .\manage.py migrate
    annuler toutes lesm girations : python manage.py migrate inscription_course zero
    Ajouter des objets facilement depuis Shell :
        python .\manage.py shell
        >>> from inscription_course.models import InscriptionCourse
        >>> insc = InscriptionCourse(nom="LE BRET",prenom="Titouan",email="22006139@etu.unicaen.fr", age = 22, course= "5km", inscription_complete = True, lien_vers_certificat = "null")
        >>> insc.save()
        #afficher tous les objets :
        >>> InscriptionCourse.objects.all()
        #recup un elemt
        >>> InscriptionCourse.objects.get(id=2)
        >>> InscriptionCourse.objects.get(nom="LE BRET") !!!il faut qu'il n'y aio qu'un LE BRET dans la BD
        #Supprimer toutes les inscriptions complete :
        >>> InscriptionCourse.objects.filter(inscription_complete=True).delete()

Pour gérer le lien entre inscription course et un compte : https://www.youtube.com/watch?v=UxTwFMZ4r5k

Pour créer un superuser (compte admin) : python manage.py createsuperuser


custom_user : générer entierement par "python manage.py create_custom_user_app"
        --> Voir doc : https://pypi.org/project/django-use-email-as-username/

inscriptions : app qui sert pour la gestion de la BDD ,gère les table InscriptionCompte et InscriptionCourse
        --> Voir le lien avec la table custom_user_user, pour lié compte et course