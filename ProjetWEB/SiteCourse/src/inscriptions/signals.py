import os
import hashlib

from django.db.models.signals import post_save, post_delete, pre_save
from django.dispatch import receiver
from django.core.mail import send_mail
from .models import InscriptionCourse, CertificatMedical
from django.conf import settings

#signal pour créer l'instance de CertificatMedical associée et renommé le fichier avec un hash
@receiver(pre_save, sender=InscriptionCourse)
def create_certificatmedical(sender, instance, **kwargs):
    certificat_name = instance.certificat_med
    # Hache le nom du fichier
    print(certificat_name)
    #Avec les deux lignes en dessous, on s'assure que même si 2 fichier ont le même nom, ils auront un hash différents
    unique_id = os.urandom(16).hex()  # Génère un identifiant unique
    hash_input = f"{certificat_name}_{unique_id}".encode('utf-8')
    hashed_name = hashlib.sha256(hash_input).hexdigest()
    new_name = f"{hashed_name}.pdf"

    # Remplace le nom du fichier par le nom haché
    instance.certificat_med.name = new_name
    instance.certificat_med_table_id = CertificatMedical.objects.create(
        user=instance.user,
        certificat_med_name=instance.certificat_med.name,
    )



#signal pour envoyé un mail de validation quand un admin valide une inscription
@receiver(post_save, sender=InscriptionCourse)
def send_validation_email(sender, instance, created, **kwargs):
    # Vérifie si l'inscription est validée par un admin et envoie un mail si c'est le cas
    if not created and instance.inscription_complete:
        # Envoyer un e-mail
        subject = "Votre inscription a été validée"
        message = f"Bonjour {instance.prenom},\n\nVotre inscription à la course {instance.course} a été validée par un administrateur. Félicitations et bonne préparation !\n\nCordialement,\nL'équipe."
        send_mail(
            subject,
            message,
            'titouanlebretuniv@gmail.com',  # Expéditeur
            [instance.user.email],  # Destinataire
        )

#signal pour supprimer les CertifcatMedical associé a l'inscription qu'on supprime
@receiver(post_delete, sender=InscriptionCourse)
def delete_certificat_intance(sender, instance, **kwargs):
    CertificatMedical.objects.filter(
        # on supprime l'instance de CertificatMedical associé dans la bdd inscriptions_certificatmedical
        user=instance.user,
        certificat_med_name=os.path.basename(instance.certificat_med.path)
        # pour n'avoir que le nom du fichier, et pas celui du dossier dans lequel il est
    ).delete()

#signal pour supprime le fichier associé a l'instance de CertifcatMedical
@receiver(post_delete, sender=CertificatMedical)
def delete_certificat_file(sender, instance, **kwargs):
    # Chemin du fichier associé à l'instance
    if instance.certificat_med_name:
        file_path = os.path.join(settings.PRIVATE_STORAGE_ROOT+'\certificats_medicaux', instance.certificat_med_name)
        if os.path.isfile(file_path):
            os.remove(file_path)
