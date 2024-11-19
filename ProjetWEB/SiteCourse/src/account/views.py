from django.contrib.auth.forms import AuthenticationForm, UserCreationForm
from django.contrib.auth import authenticate, login, logout
from django.contrib import messages
from django.shortcuts import render, redirect
from django import forms
from django.contrib.auth import get_user_model

#Pour utiliser notre modèle d'user perso de custom_user
User = get_user_model()
class EmailUserCreationForm(UserCreationForm):
    class Meta:
        model = User
        fields = ('email', 'password1', 'password2')  # Utilisation de l'email et des mots de passe

    email = forms.EmailField(label='Email', max_length=254)  # Champ pour l'email

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

def logout_user(request):
    logout(request)
    return redirect('accueil')

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