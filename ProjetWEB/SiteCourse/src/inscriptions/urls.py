from django.urls import path, include
from . import views
#from siteCourse.views import inscriptions


#Défini le nom de l'application, que  l'on utilisera donc dans "siteCourse"
app_name = "inscriptions"

urlpatterns = [
    # Page d'accueil, redirige vers la vue 'inscriptions'
    # --> Cette page peut etre appelé dans siteCourse par : inscriptions:home
    path('', views.inscriptions , name='home'),

    path('supprimer/', views.supprimer_inscription, name='supprimer_inscription'),

    #Peut être utile plus tard lors de la gestion du paiement
    path('paiement/', views.paiement , name='paiement'),
    #urls fourni avec la bibliothèque paypal
    path('payment_success', views.payment_success, name='payment_success'),
    path('payment_failed', views.payment_failed, name='payment_failed'),
    path('paypal', include("paypal.standard.ipn.urls")),
]