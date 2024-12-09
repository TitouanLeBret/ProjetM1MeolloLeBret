import os

from django.http import Http404, HttpResponse
from django.shortcuts import redirect, render ,get_object_or_404, redirect
from .models import InscriptionCourse, CertificatMedical
from django.contrib import messages
#import pour paypal
from django.urls import reverse
from paypal.standard.forms import PayPalPaymentsForm
from django.conf import settings
import uuid #permet de creer un user id unique pour ne pas faire de paiement en double

from .forms import InscriptionForm

#import magic  # Pour vérifier le type MIME (donc le type de fichier, pdf,jpeg,png ...)
from django.core.exceptions import ValidationError


def inscriptions(request):
    # Vérification si l'utilisateur est authentifié
    if request.user.is_authenticated:
        # Récupération des inscriptions précédentes de l'utilisateur en filtrant par user id
        inscriptions = InscriptionCourse.objects.filter(
            user_id = request.user.id,
        )
        #Traitement du formulaire lorsqu'une qu'il est soumis
        if request.method == "POST":
            form = InscriptionForm(request.POST, request.FILES) #Création du formulaire avec les données soumises
            if form.is_valid():
                """Partie paiement"""
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

                """Partie inscription"""
                insc = form.save(commit=False)
                if request.user.is_authenticated:
                    insc.user = request.user
                    insc.certificat_med_table_id = CertificatMedical.objects.create(
                        user=request.user,
                        certificat_med_name = form.cleaned_data.get('certificat_med'),
                    )
                insc.save()
                messages.success(request, "Votre inscription a bien été prise en compte.")
                return render(request, 'inscriptions/insc_complete.html',{'paypal_form': paypal_form, 'insc': insc, 'inscriptions': inscriptions})
        else:
            # Si pas de soumission POST, on créer formulaire vierge et on affiche la page
            form = InscriptionForm()
        return render(request, 'inscriptions/accueil.html', {'form': form , 'inscriptions' : inscriptions})
    else :

        messages.error(request,"Pour vous inscrire a une course, vous devez être connecté")
        return redirect('/accounts')

#Fonction de suppression de l'inscription, qui supprime également le certificat médicale associé
# (instance dans la db inscriptions_certifcatmedical et le fichier pdf dans private_storage/certificats_medicaux
def supprimer_inscription(request):
    if request.method == 'POST':
        inscription_id = request.POST.get('inscription_id')
        inscription = InscriptionCourse.objects.filter(id=inscription_id).first() #le first ici sert a avoir un elt et pas une liste
        if inscription:
            CertificatMedical.objects.filter( #on supprime l'instance de CertificatMedical associé dans la bdd inscriptions_certificatmedical
                user=inscription.user,
                certificat_med_name=os.path.basename(inscription.certificat_med.path) #pour n'avoir que le nom du fichier, et pas celui du dossier dans lequel il est
            ).delete()
            inscription.delete()
            messages.success(request, "L'inscription a été supprimée avec succès.")
        else:
            #si l'inscription n'existe pas
            messages.error(request, "L'inscription que vous tentez de supprimer n'existe pas.")

        return redirect('inscriptions:home')
    return redirect('inscriptions:home')