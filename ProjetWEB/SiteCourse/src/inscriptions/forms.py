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

    #Validation de base du pdf, juste que le fichier finisse bien par .pdf, mais ne sécurise rien
    def clean_certificat_med(self):
        certificat = self.cleaned_data.get('certificat_med')
        if not certificat:
            raise forms.ValidationError("Le certificat médical est obligatoire.")
        if certificat and not certificat.name.endswith('.pdf'):
            raise forms.ValidationError("Le fichier doit être au format PDF.")
        return certificat


"""
#Création de notre formulaire d'inscription a une course a partie d'un formulaire Django
class InscriptionForm(forms.Form):
    # Définition des champs du formulaire d'inscription
    nom = forms.CharField(max_length=100)
    prenom = forms.CharField(max_length=100)
    age = forms.IntegerField(min_value=1, max_value=110)
    course = forms.ChoiceField(choices=[('5km', '5 km'), ('10km', '10 km'), ('semi-marathon', 'Semi-marathon'), ('marathon', 'Marathon')])
    captcha = ReCaptchaField(widget=ReCaptchaV2Checkbox())
    certificat_medical = forms.FileField(
        label="Certificat médical (PDF uniquement)",
        required=False,
        widget=forms.ClearableFileInput(attrs={'accept': '.pdf'})
    )

    def clean_certificat_medical(self):
        fichier = self.cleaned_data['certificat_medical']

        # Vérification de la taille
        if fichier.size > 5 * 1024 * 1024:  # 5 Mo
            raise ValidationError("Le fichier est trop volumineux (max. 5 Mo).")

        # Vérification du type MIME
        mime = magic.Magic(mime=True)
        if mime.from_buffer(fichier.read()) != "application/pdf":
            raise ValidationError("Le fichier doit être au format PDF.")

        # Repositionner le pointeur au début du fichier
        fichier.seek(0)

        return fichier"""
