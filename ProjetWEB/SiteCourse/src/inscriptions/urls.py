from django.urls import path
from . import views
#from siteCourse.views import inscriptions


app_name = "inscriptions"

urlpatterns = [
    path('', views.inscriptions , name='home'),
    path('insc_complete/', views.insc_complete , name='complete'),
    path('insc_failed/', views.insc_failed , name='failed'),
    path('paiement/', views.paiement , name='paiement'),
    #path('mdpchange,.../......) etc
]