# Generated by Django 5.1.3 on 2024-12-04 13:45

from django.db import migrations


class Migration(migrations.Migration):

    dependencies = [
        ('inscriptions', '0006_alter_inscriptioncourse_certificat_med'),
    ]

    operations = [
        migrations.RemoveField(
            model_name='inscriptioncourse',
            name='certificat_med',
        ),
    ]