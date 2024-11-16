from django.shortcuts import render

# Create your views here.

def insc_complete(request):
    return render (request, 'inscriptions/insc_complete.html', )

def insc_failed(request):
    return render (request, 'inscriptions/insc_failed.html', )

def paiement(request):
    return render (request, 'inscriptions/paiement.html', )