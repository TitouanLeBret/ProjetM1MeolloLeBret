from django.urls import path
from . import views
from siteCourse.views import account


app_name = "account"

urlpatterns = [
    path('', account , name='home'),
    path('login/', views.login_user , name='login'),
    path('register/', views.register_user , name='register'),
    path('logout/', views.logout_user , name='logout'),
    #path('mdpchange,.../......) etc
]