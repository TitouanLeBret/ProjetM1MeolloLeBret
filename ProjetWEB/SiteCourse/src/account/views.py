"""
Vues pour la gestion des utilisateurs dans l'application 'account' :

- Connexion : Permet à l'utilisateur de se connecter avec son email et son mot de passe.
- Déconnexion : Permet à l'utilisateur de se déconnecter.
- Inscription : Permet à un nouvel utilisateur de s'inscrire avec son email et un mot de passe.

"""

from django.contrib.auth.forms import AuthenticationForm, UserCreationForm
from django.contrib.auth import authenticate, login, logout
from django.contrib import messages
from django.shortcuts import render, redirect
from django import forms
from django.contrib.auth import get_user_model

"""
Utilisation du modèle d'utilisateur personnalisé (custom user), définis dans l'app custom_user dans models.py
Ce custom user vient du module Django django_use_email_as_username
"""
User = get_user_model()

"""
Formulaire personnaliser de création de compte utilisateur avec email
Ce formulaire est une modification d'un formulaire django

-Champs 1 : Email 
-Champs 2 : Mot de passe
-Champs 3 : Verification mot de passe
"""
class EmailUserCreationForm(UserCreationForm):
    class Meta:
        model = User
        fields = ('email', 'password1', 'password2')  # Utilisation de l'email et des mots de passe

    #Renome le champs email en "Email", de base s'appelle "Adresse électronique"
    email = forms.EmailField(label='Email', max_length=254)  # Champ pour l'email



"""
Fonction pour la vue de login

Si la requete est de type POST
    Verifie que l'utilisateur existe et le connecte si c'est le cas
    Sinon renvoie une erreur dans le formulaire
S'il n'y a pas de requete 
    Renvoie un formulaire de base AuthenticationForm() 
    PS : ce formulaire AuthenticationForm prend automatiquement en compte notre custom_user et a donc un champs email et password
"""
def login_user(request):
    if request.method=="POST" :
        username = request.POST['username']
        password = request.POST['password']

        user = authenticate(request, username=username, password=password)

        if user is not None:
            login(request, user)
            return redirect('accueil')
        else :
            messages.info(request, 'Identifiant et/ou mot de passe incorrect')
    form = AuthenticationForm()
    return render(request, 'account/login.html', {'form': form})


"""
Fonction de déconnexion
Déconnecte l'utilisateur et renvoie vers l'accueil

A FAIRE PEUT ETRE :
    Vérifier que l'utilisateur est connecté 
    Tester ce qu'il se passe sinon
"""
def logout_user(request):
    logout(request)
    return redirect('accueil')


"""
Fonction pour la vue de register

Si la requete est de type POST
    Test si le formulaire est valide 
    --> la fonction is_valid() viens de Django, et comme nous avons définis champs email et password
    --> vérifie donc email valide (présence d'un '@' et d'un '.')
    --> vérifie que les 2 mots de passe correspondent et qu'ils respectent les règles de sécurités (8 carac, pas mdp courant, pas entierement numérique, pas trop semblable a infos perso (email)) 
S'il n'y a pas de requete 
    Renvoie un formulaire EmailUserCreationForm() (structure que l'on a définis plus haut)
"""
def register_user(request):
    if request.method == "POST":
        form = EmailUserCreationForm(request.POST)
        if form.is_valid():
            user = form.save()
            login(request,user)
            return redirect("accueil")

    else :
        form = EmailUserCreationForm()
    return render (request, 'account/register.html', {'form': form})