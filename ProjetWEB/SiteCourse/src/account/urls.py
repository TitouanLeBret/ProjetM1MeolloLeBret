"""
Configuration des URLs pour l'application 'account' de notre site

Ce module associe chaque URL à une fonction de vue spécifique.
"""

from django.urls import path
from . import views

#Défini le nom de l'application, que  l'on utilisera donc dans "siteCourse"
app_name = "account"

urlpatterns = [
    # Page d'accueil, redirige vers la vue 'account'
    path('', views.account , name='home'),

    # Page de suppresion de compte
    path('delete/', views.delete_account, name='delete_account'),

    # Page de changement d'email
    path('change_email/', views.change_email, name='change_email'),

    # Page de changement de mot de passe
    path('change_password/', views.change_password, name='change_password'),

    # Page de connexion, redirige vers la vue 'login_user'
    path('login/', views.login_user , name='login'),

    # Page d'inscription, redirige vers la vue 'register_user'
    path('register/', views.register_user , name='register'),

    # Page de déconnexion, redirige vers la vue 'logout_user'
    path('logout/', views.logout_user , name='logout'),

    # TODO : Ajouter d'autres URL si nécessaire
    #path('mdpchange,.../......) etc
]