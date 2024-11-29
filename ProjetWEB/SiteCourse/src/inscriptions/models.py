from django.db import models
from django.contrib.auth import get_user_model

User = get_user_model()

# Modèle représentant une inscription à une course.
class InscriptionCourse(models.Model):
    """
       Ce modèle stocke les informations relatives à l'inscription d'un participant à une course.
    """
    user = models.ForeignKey(User, on_delete=models.CASCADE,null=True)
    nom = models.CharField(max_length=255)
    prenom = models.CharField(max_length=255)
    age = models.IntegerField()
    course = models.CharField(max_length=255)
    # Statut de l'inscription (complète ou non)
    inscription_complete = models.BooleanField()
    # Lien vers un certificat médical (lien vers la BDD ? ou vers un hebergement pdf en ligne ?)
    lien_vers_certificat = models.CharField(max_length=2048)
    # Date de l'inscription, ajoutée automatiquement à la création de l'objet
    inscription_date = models.DateField(auto_now_add=True)
    #paiement_complet = models.BooleanField()
    #compte = models.ForeignKey('InscriptionCompte', null=False, on_delete=models.CASCADE)

    """
        Fonction pour l'affichage d'une Inscription (Utile pour le débug)
    """
    def __str__(self):
        status = "Complète" if self.inscription_complete else "Incomplète"
        return f"Inscription: {self.prenom} {self.nom} ({self.age} ans) - Course: {self.course} - Statut: {status} "


    """
        Définis le nom de la table dans notre base de donnée
    """
    class Meta:
        db_table = "InscriptionCourse"  # Nom exact de la table dans la base de données
