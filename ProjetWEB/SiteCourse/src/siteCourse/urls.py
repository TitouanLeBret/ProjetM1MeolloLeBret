"""
URL configuration for siteCourse project.

The `urlpatterns` list routes URLs to views. For more information please see:
    https://docs.djangoproject.com/en/5.1/topics/http/urls/
Examples:
Function views
    1. Add an import:  from my_app import views
    2. Add a URL to urlpatterns:  path('', views.home, name='home')
Class-based views
    1. Add an import:  from other_app.views import Home
    2. Add a URL to urlpatterns:  path('', Home.as_view(), name='home')
Including another URLconf
    1. Import the include() function: from django.urls import include, path
    2. Add a URL to urlpatterns:  path('blog/', include('blog.urls'))
"""
from django.contrib import admin
from django.urls import path, include
from .views import accueil, parcours, login_page
from inscriptions.views import inscriptions

urlpatterns = [
#IMPORTANT ; Penser a modifier /admin en autre chose pour que les gens ne puissnet pas attérir sur la page d'admin du site
#expliquer ce choiix dans rapport :
    #si failles sur django admin, les gens ne peuvent pas trouver notre page automatiquement, donc compléxifie la chose
    path('admin_gestion/', admin.site.urls),
    path('', accueil , name='accueil'),
    path('accueil/', accueil , name='accueil'),
    path('inscriptions/' , include('inscriptions.urls')),
    path('parcours/' , parcours ,  name='parcours'),
    path('account/', include('account_own.urls') ), #quand app on ne peut pas mettre name
    path('login/', login_page , name='login_page'),
    #path('inscriptions/', include('inscriptions.urls')), pour avoir inscriptions/...

    path('captcha/', include('captcha.urls')),

    path('accounts/', include('allauth.urls')),
]
