from django.shortcuts import render, redirect
from inscriptions.models import InscriptionCourse


def accueil(request):
    return render(request, 'accueil.html')

def parcours(request):
    return render(request, 'parcours.html')

def login_page(request):
    return render(request, 'login_page.html')

def account(request):
    return render(request, 'account.html')