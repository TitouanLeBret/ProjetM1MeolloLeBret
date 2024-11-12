from django.shortcuts import render

def accueil(request):
    return render(request, 'accueil.html')

def inscriptions(request):
    return render(request, 'inscriptions.html')

def parcours(request):
    return render(request, 'parcours.html')

def login_page(request):
    return render(request, 'login_page.html')

def compte(request):
    return render(request, 'compte.html')