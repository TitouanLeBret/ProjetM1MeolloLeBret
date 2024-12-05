import hashlib
import os

from django import forms
from django_recaptcha.fields import ReCaptchaField
from django_recaptcha.widgets import ReCaptchaV2Checkbox
from .models import InscriptionCourse, CertificatMedical


class InscriptionForm(forms.ModelForm):
    captcha = ReCaptchaField(widget=ReCaptchaV2Checkbox())  # Champ supplémentaire non lié au modèle

    class Meta:
        model = InscriptionCourse
        fields = ['nom', 'prenom', 'age', 'course', 'certificat_med']
        widgets = {
            'certificat_med': forms.FileInput(attrs={'accept': '.pdf'}),
        }

    #Validation de base du pdf, juste que le fichier finisse bien par .pdf, mais ne sécurise rien !!!!!!!!!!!
    #A FAIRE : SECURISER LE PDF
    def clean_certificat_med(self):
        certificat = self.cleaned_data.get('certificat_med')
        if certificat:
        # Hache le nom du fichier
            original_name = certificat.name
            print(original_name)
            #Avec les deux lignes en dessous, on s'assure que même si 2 fichier ont le même nom, ils auront un hash différents
            unique_id = os.urandom(16).hex()  # Génère un identifiant unique
            hash_input = f"{original_name}_{unique_id}".encode('utf-8')
            hashed_name = hashlib.sha256(hash_input).hexdigest()
            new_name = f"{hashed_name}.pdf"

        # Remplace le nom du fichier par le nom haché
            certificat.name = new_name
        return certificat