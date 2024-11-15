from django.shortcuts import render, redirect
from inscription_course.models import InscriptionCourse


def accueil(request):
    return render(request, 'accueil.html')

def inscriptions(request):
    if request.method == "POST":
        nom = request.POST['nom']
        prenom = request.POST['prenom']
        email = request.POST['email']
        age = request.POST['age']
        course = request.POST['course']

        # Crée une nouvelle instance de l'inscription dans la BDD
        #inscription = Inscription(nom=nom, prenom=prenom, email=email, age=age, course=course)
        #inscription.save()

        """Faire des test pour que tout soit valide"""
        """Inscritpion dans BDD"""
        insc = InscriptionCourse(nom=nom,
                                 prenom=prenom,
                                 email=email,
                                 age=age,
                                 course=course,
                                 inscription_complete=True,
                                 lien_vers_certificat="null")
        insc.save()
        #print(InscriptionCourse.objects.all())

        # Redirige vers une page de confirmation
        context = {
            'nom': nom,
            'prenom': prenom,
            'email': email,
            'age': age,
            'course': course,
            'inscription_complete': True  # Indicateur pour afficher le message
        }
        return render(request, 'inscriptions.html', context)
    return render(request, 'inscriptions.html')

def parcours(request):
    return render(request, 'parcours.html')

def login_page(request):
    return render(request, 'login_page.html')

def compte(request):
    return render(request, 'compte.html')