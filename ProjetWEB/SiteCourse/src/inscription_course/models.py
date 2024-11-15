from django.db import models

# Create your models here.

class InscriptionCourse(models.Model):
    nom = models.CharField(max_length=255)
    prenom = models.CharField(max_length=255)
    email = models.CharField(max_length=255)
    age = models.IntegerField()
    course = models.CharField(max_length=255)
    inscription_complete = models.BooleanField()
    lien_vers_certificat = models.CharField(max_length=2048)
    inscription_date = models.DateField(auto_now_add=True)
    #compte = models.ForeignKey('InscriptionCompte', null=False, on_delete=models.CASCADE)

    def __str__(self):
        status = "Complète" if self.inscription_complete else "Incomplète"
        return f"Inscription: {self.prenom} {self.nom} ({self.age} ans) - Course: {self.course} - Statut: {status} - Email: {self.email}"

    class Meta:
        db_table = "InscriptionCourse"  # Nom exact de la table dans la base de données

"""
#A mettre dans une app inscritpion_compte ???????
class InscriptionCompte(models.Model):
    nom = models.CharField(max_length=255,null=False)
"""