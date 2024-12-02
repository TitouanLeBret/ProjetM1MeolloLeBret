from django.shortcuts import redirect, render ,get_object_or_404, redirect
from .models import InscriptionCourse
from django.contrib import messages
from django import forms
from captcha.fields import CaptchaField
#import pour paypal
from django.urls import reverse
from paypal.standard.forms import PayPalPaymentsForm
from django.conf import settings
import uuid #permet de creer un user id unique pour ne pas faire de paiement en double

#Création de notre formulaire d'inscription a une course a partie d'un formulaire Django
class InscriptionForm(forms.Form):
    # Définition des champs du formulaire d'inscription
    nom = forms.CharField(max_length=100)
    prenom = forms.CharField(max_length=100)
    age = forms.IntegerField(min_value=1, max_value=110)
    course = forms.ChoiceField(choices=[('5km', '5 km'), ('10km', '10 km'), ('semi-marathon', 'Semi-marathon'), ('marathon', 'Marathon')])
    captcha = CaptchaField()

def inscriptions(request):
    # Vérification si l'utilisateur est authentifié pour
    # donner a la page ses inscriptions si c'est le cas
    if request.user.is_authenticated:
        # Récupération des inscriptions précédentes de l'utilisateur en filtrant par user id
        # ! ! ! ! ! ! ! ! !
        # ! ! ! ! ! ! ! ! !
        # A modifié par le user_id peut etre
        # ! ! ! ! ! ! ! ! !
        # ! ! ! ! ! ! ! ! !
        inscriptions = InscriptionCourse.objects.filter(
            user_id = request.user.id,
        )
    else :
        # Si l'utilisateur n'est pas authentifié, on initialise une variable vide pour les inscriptions
        inscriptions = ()


    # Traitement du formulaire lorsqu'une qu'il est soumis (requete POST)
    if request.method == "POST":
        form = InscriptionForm(request.POST) # Création du formulaire avec les données soumises
        if form.is_valid():
            course = form.cleaned_data['course']
            host = request.get_host()
            price = 5
            if course == "5km": price = 10
            if course == "10km": price = 20
            if course == "semi-marathon": price = 40
            if course == "marathon": price = 80

            # dictionnaire de formulaire paypal
            paypal_dict = {
                'business': settings.PAYPAL_RECEIVER_EMAIL,
                'amount': price,  # prix de la course
                'item_name': 'paiement de course',
                'no_shipping': '2',  # permet de choisir son adresse
                'invoice': str(uuid.uuid4()),
                'currency_code': 'EUR',
                # 'notify_url': 'https://{}{}'.format(host, reverse("paypal-ipn")),
                # 'return_url': 'https://{}{}'.format(host, reverse("payment_success")),
                # 'cancel_return': 'https://{}{}'.format(host, reverse("payment_failed")),
            }
            # formulaire paypal
            paypal_form = PayPalPaymentsForm(initial=paypal_dict)
        if form.is_valid(): # Vérification de la validité des données soumises
            #Sauvegarde dans BDD sécurisé grâce à cleaned_data
            # ! ! ! ! ! ! ! ! !
            # ! ! ! ! ! ! ! ! !
            # Voir exactement l'effet de cleaned_data (pour expliquer dans rapport):
            # ! ! ! ! ! ! ! ! !
            # ! ! ! ! ! ! ! ! !
            if request.user.is_authenticated:
                insc = InscriptionCourse.objects.create(
                    user = request.user,
                    nom=form.cleaned_data['nom'],
                    prenom=form.cleaned_data['prenom'],
                    age=form.cleaned_data['age'],
                    course=form.cleaned_data['course'],
                    inscription_complete = True
                )
            else :
                insc = InscriptionCourse.objects.create(
                    nom=form.cleaned_data['nom'],
                    prenom=form.cleaned_data['prenom'],
                    age=form.cleaned_data['age'],
                    course=form.cleaned_data['course'],
                    inscription_complete=True
                )
            human = True #form_is valid verifie le captcha et ici on dit bien qu'il a était validé
            # Redirection vers une page de succès avec les infos de l'inscriptions a afficher
            messages.success(request,"Votre inscription a bien était prise en compte")
            #return redirect('accueil')
            return render(request, 'inscriptions/insc_complete.html',{'paypal_form': paypal_form, 'insc': insc, 'inscriptions': inscriptions})
        else:
            # Si le formulaire est invalide, on renvoie la page d'accueil avec les erreurs du formulaire
            # Ce cas n'arrive jamais je penses, car le POST n'est effectué que si les données sont valides
            # Mais laisser pour sécurité maximum ?
            return render(request, 'inscriptions/accueil.html', {'form': form , 'inscriptions' : inscriptions})
    else:
        # Si pas de soumission POST, on créer formulaire vierge et on affiche la page
        form = InscriptionForm()
        return render(request, 'inscriptions/accueil.html', {'form': form , 'inscriptions' : inscriptions})



def supprimer_inscription(request):
    if request.method == 'POST':
        inscription_id = request.POST.get('inscription_id')
        inscription = InscriptionCourse.objects.filter(id=inscription_id) #le first ici sert a avoir un elt et pas une liste

        if inscription:
            # Si l'inscription existe, supprimer
            inscription.delete()

            # Ajouter un message de confirmation
            messages.success(request, "L'inscription a été supprimée avec succès.")
        else:
            # Si l'inscription n'existe pas
            messages.error(request, "L'inscription que vous tentez de supprimer n'existe pas.")

        return redirect('inscriptions:home')

        # Si ce n'est pas une requête POST, rediriger vers la page des inscriptions
    return redirect('inscriptions:home')



#Vue a implémenter pour le systeme de paiement (si celui ci n'est pas sur la page d'inscriptions directement
def paiement(request):
    return render (request, 'inscriptions/paiement.html', )

def payment_success(request):
    return render(request, "payment/payment_success.html",{})

def payment_failed(request):
    return render(request, "payment/payment_failed.html",{})