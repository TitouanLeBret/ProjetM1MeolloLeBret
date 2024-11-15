from django.contrib.auth.forms import AuthenticationForm, UserCreationForm
from django.contrib.auth import authenticate, login, logout
from django.contrib import messages
from django.shortcuts import render, redirect

# Create your views here.

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
        form = UserCreationForm(request.POST)
        if form.is_valid():
            form.save()
            return redirect("accueil")

    else :
        form = UserCreationForm()

    return render (request, 'account/register.html', {'form': form})