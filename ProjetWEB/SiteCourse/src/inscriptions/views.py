from django.shortcuts import redirect, render
from .models import InscriptionCourse
from django import forms


#Création de notre formulaire d'inscription a une course a partie d'un formulaire Django
class InscriptionForm(forms.Form):
    nom = forms.CharField(max_length=100)
    prenom = forms.CharField(max_length=100)
    email = forms.EmailField()
    age = forms.IntegerField(min_value=1)
    course = forms.ChoiceField(choices=[('5km', '5 km'), ('10km', '10 km'), ('semi-marathon', 'Semi-marathon'), ('marathon', 'Marathon')])


def inscriptions(request):
    if request.method == "POST":
        form = InscriptionForm(request.POST)
        if form.is_valid():
            #Sauvegarde sécurisé grâce à cleaned_data
            InscriptionCourse.objects.create(
                nom=form.cleaned_data['nom'],
                prenom=form.cleaned_data['prenom'],
                email=form.cleaned_data['email'],
                age=form.cleaned_data['age'],
                course=form.cleaned_data['course'],
                inscription_complete = True
            )
            return redirect('inscriptions:complete')  # Redirection vers une page de succès
        else:
            print("loupé")
            return render(request, 'inscriptions/accueil.html', {'form': form})
    else:
        form = InscriptionForm()
        return render(request, 'inscriptions/accueil.html', {'form': form})

def insc_complete(request):
    return render (request, 'inscriptions/insc_complete.html', )

def insc_failed(request):
    return render (request, 'inscriptions/insc_failed.html', )

def paiement(request):
    return render (request, 'inscriptions/paiement.html', )