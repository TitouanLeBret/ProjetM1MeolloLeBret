from django import forms
from django.shortcuts import render, redirect
from inscriptions.models import InscriptionCourse




#Création de notre formulaire de modification de compte 
#Il faut ajotuer : 
# -modification mots de passe
# -gérer ajout certif med et modifs de celui-ci (on peut le consulter)
# -modifs de l'email (doit donc modifier la connexion au compte)
# -autres modifs ??????
class AccountForm(forms.Form):
    # Définition des champs du formulaire d'inscription
    prenom = forms.CharField(max_length=100, required=False)
    nom = forms.CharField(max_length=100, required=False)
    #email = forms.EmailField()
    age = forms.IntegerField(min_value=1, max_value=110, required=False)





def accueil(request):
    return render(request, 'accueil.html')

def parcours(request):
    return render(request, 'parcours.html')

def login_page(request):
    return render(request, 'login_page.html')

def account(request):
    # Traitement du formulaire lorsqu'une qu'il est soumis (requete POST)
    if request.method == "POST":
        form = AccountForm(request.POST) # Création du formulaire avec les données soumises
        if form.is_valid(): # Vérification de la validité des données soumises
            user = request.user
            user.prenom = request.POST.get('prenom')
            user.nom = request.POST.get('nom')
            user.age = request.POST.get('age')
            # Enregistre les modifications dans la base de données
            user.save()
            # Redirection vers la même page (PEUT ETRE A MODIFIER !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!)
            return render(request, 'account.html', {'form': form , 'success': True})
        else:
            # Si le formulaire est invalide, on renvoie la page d'accueil avec les erreurs du formulaire
            # Ce cas n'arrive jamais je penses, car le POST n'est effectué que si les données sont valides
            # Mais laisser pour sécurité maximum ?
            return render(request, 'account.html', {'form': form })
    else:
        # Si pas de soumission POST, on créer formulaire vierge et on affiche la page
        if request.user.is_authenticated :
        #Si connecté : 
        # ce formulaire sera pré rempli avec les valeurs de la base grâce a initial=initial_data 
            initial_data = {
                'prenom': request.user.prenom,
                'nom': request.user.nom,
                'age': request.user.age,
            }  # Rempli avec les données de l'utilisateur connecté
            form = AccountForm(initial=initial_data)
            return render(request, 'account.html', {'form': form})
        #si pas connecté
        # Formulaire vide
        else :
            form = AccountForm()
            return render(request, 'account.html', {'form': form})