from django_use_email_as_username.models import BaseUser, BaseUserManager


"""
Fichier générer automatiquement par python manage.py create_custom_user_app
Voir doc : https://pypi.org/project/django-use-email-as-username/
"""
class User(BaseUser):
    objects = BaseUserManager()
