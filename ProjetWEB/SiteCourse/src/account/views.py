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


******** PARTIE CONNEXION A UN COMTPE ********


"""




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


******** PARTIE DECONNEXION DU COMTPE ********


"""






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


******** PARTIE CREATION DU COMPTE ********


"""


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




"""


******** PARTIE AFFICHAGE DE LA VU DU COMPTE ET MODIFS INFO (NOM,PRENOM,AGE,...) ********


"""


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






"""
Fonction pour la vue du compte

A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

"""
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
            return render(request, 'account/account.html', {'form': form , 'success': True})
        else:
            # Si le formulaire est invalide, on renvoie la page d'accueil avec les erreurs du formulaire
            # Ce cas n'arrive jamais je penses, car le POST n'est effectué que si les données sont valides
            # Mais laisser pour sécurité maximum ?
            return render(request, 'account/account.html', {'form': form })
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
            return render(request, 'account/account.html', {'form': form})
        #si pas connecté
        # Formulaire vide
        else :
            form = AccountForm()
            return render(request, 'account/account.html', {'form': form})





"""


******** PARTIE SUPPRESION DU COMPTE ********


"""

"""
Formulaire personnaliser de changement d'email
Ce formulaire est une modification d'un formulaire django

-Champs 1 : Email
-Champs 2 : Mot de passe

A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

"""
class UserDeleteAccountForm(forms.Form):
    email = forms.EmailField(label='Email', max_length=254)
    password = forms.CharField(label='Mot de passe', widget=forms.PasswordInput)

#Explication de pourquoi *args et **kwargs sur stackoverflow : https://stackoverflow.com/questions/871037/django-overriding-init-for-custom-forms
    def __init__(self, user ,*args, **kwargs):
        self.user = user #Donner l'utilisateur actif
        super().__init__(*args, **kwargs)

# Fonction clean_xxx appelé automatiquement par django lors de is_valid()
    def clean_email(self):
        email = self.cleaned_data.get('email')
        if email != self.user.email:
            raise forms.ValidationError("L'email ne correspond pas.")
        return email






"""
Fonction pour suppression d'un compte

A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

"""

def delete_account(request):
    if request.method == "POST":
        form = UserDeleteAccountForm(request.user,request.POST)
        if form.is_valid():
            email = form.cleaned_data.get('email')
            password = form.cleaned_data.get('password')

            user = authenticate(request, email=email, password=password)
            if user:
                user.delete()
                messages.success(request, "Votre compte a été supprimé avec succès.")
                return redirect('account:login')  # Rediriger vers la page de connexion
        else:
            messages.error(request, "Les informations saisies sont incorrectes ou ne correspondent pas a votre compte.")
    else :
        form = UserDeleteAccountForm(request.user)
    return render(request, 'account/delete_account.html', {'form': form})




"""


******** PARTIE CHANGEMENT DE L'EMAIL ********


"""



"""
Formulaire personnaliser de changement d'email
Ce formulaire est une modification d'un formulaire django

-Champs 1 : Ancien Email 
-Champs 2 : Mot de passe
-Champs 3 : Nouvel email

A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

"""
class UserChangeMailForm(forms.Form):
    old_email = forms.EmailField(label='Ancien Email', max_length=254)
    password = forms.CharField(label='Mot de passe', widget=forms.PasswordInput)
    new_email = forms.EmailField(label='Nouvel Email', max_length=254)

#Explication de pourquoi *args et **kwargs sur stackoverflow : https://stackoverflow.com/questions/871037/django-overriding-init-for-custom-forms
    def __init__(self, user ,*args, **kwargs):
        self.user = user  #Donner l'utilisateur actif
        super().__init__(*args, **kwargs)


#Fonction clean_xxx appelé automatiquement par django lors de is_valid()
    def clean_old_email(self):
        old_email = self.cleaned_data.get('old_email')
        if old_email != self.user.email:
            raise forms.ValidationError("L'ancien email ne correspond pas à celui associé à votre compte.")
        return old_email

    def clean_new_email(self):
        new_email = self.cleaned_data.get('new_email')
        if User.objects.filter(email=new_email).exists():
            raise forms.ValidationError("Cet email est déjà utilisé.")
        return new_email


"""
Fonction pour le changement d'email d'un compte

A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

"""
def change_email(request):
    if request.method == "POST":
        form = UserChangeMailForm(request.user,request.POST)
        if form.is_valid():
            old_email = form.cleaned_data.get("old_email")
            password = form.cleaned_data.get("password")
            new_email = form.cleaned_data.get("new_email")
            user = authenticate(request, email=old_email, password=password)
            if user:
                user.email = new_email
                user.save()
                messages.success(request, "Votre email a été mis à jour.")
                return redirect('account:home')  # Rediriger vers la page compte
            else:
                messages.error(request, "Les informations saisies sont incorrectes.")
    else :
        form = UserChangeMailForm(request.user)
    return render(request, 'account/change_email.html', {'form': form})


"""


******** PARTIE CHANGEMENT DU MOT DE PASSE ********


"""






"""
Formulaire personnaliser de changement de mot de passe
Ce formulaire est une modification d'un formulaire django

-Champs 1 : Email
-Champs 2 : Mot de passe actuel
-Champs 3 : Nouveau Mot de passe

A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

"""
class UserChangePasswordForm(forms.Form):
    email = forms.EmailField(label='Ancien Email', max_length=254)
    old_password = forms.CharField(label='Mot de passe', widget=forms.PasswordInput)
    new_password = forms.CharField(label='Mot de passe', widget=forms.PasswordInput)

#Explication de pourquoi *args et **kwargs sur stackoverflow : https://stackoverflow.com/questions/871037/django-overriding-init-for-custom-forms
    def __init__(self, user ,*args, **kwargs):
        self.user = user # Passez l'utilisateur lors de l'initialisation
        super().__init__(*args, **kwargs)

# Fonction clean_xxx appelé automatiquement par django lors de is_valid()

    """Cette fonction est impossible car compare avec le HASH
    def clean_old_password(self):
        old_password = self.cleaned_data.get('old_password')
        print("JE SUIS LAAAAAA")
        if old_password != self.user.password:
            print(self.user.password)
            raise forms.ValidationError("L'ancien mot de passe ne correspond pas à celui associé à votre compte.")
        return old_password
    """


    """
    def clean_new_password(self):
        new_password = self.cleaned_data.get('new_password')
        #FAIRE DES TEST SUR LE MOTS DE PASSE ?
        #LOGIQUEMENT NON CAR DEJA FAIT PAR widget=forms.PasswordInput
        return new_password
        
"""



"""
Fonction pour le changement de mot de passe d'un compte

A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
A FAIRE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

"""
def change_password(request):
    if request.method == "POST":
        form = UserChangePasswordForm(request.user,request.POST)
        if form.is_valid():
            email = form.cleaned_data.get('email')
            old_password = form.cleaned_data.get('old_password')
            new_password = form.cleaned_data.get('new_password')

            user = authenticate(request, email=email, password=old_password)
            if user:
                user.set_password(new_password)
                user.save()
                messages.success(request, "Votre mot de passe a été changé.")
                login(request, user)  # Reconnexion automatique
                return redirect('account:home')
            else:
                messages.error(request, "Les informations saisies sont incorrectes.")
    else:
        form = UserChangePasswordForm(request.user)
    return render(request, 'account/change_password.html', {'form': form})