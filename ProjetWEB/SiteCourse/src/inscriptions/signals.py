import os

from django.db.models.signals import post_save, post_delete
from django.dispatch import receiver
from django.core.mail import send_mail
from .models import InscriptionCourse, CertificatMedical
from django.conf import settings


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

#signal pour supprime le fichier associé a l'instance de CertifcatMedical
@receiver(post_delete, sender=CertificatMedical)
def delete_certificat_file(sender, instance, **kwargs):
    # Chemin du fichier associé à l'instance
    if instance.certificat_med_name:
        file_path = os.path.join(settings.PRIVATE_STORAGE_ROOT+'\certificats_medicaux', instance.certificat_med_name)
        if os.path.isfile(file_path):
            os.remove(file_path)
