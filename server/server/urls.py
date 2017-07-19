"""server URL Configuration

The `urlpatterns` list routes URLs to views. For more information please see:
    https://docs.djangoproject.com/en/1.10/topics/http/urls/
Examples:
Function views
    1. Add an import:  from my_app import views
    2. Add a URL to urlpatterns:  url(r'^$', views.home, name='home')
Class-based views
    1. Add an import:  from other_app.views import Home
    2. Add a URL to urlpatterns:  url(r'^$', Home.as_view(), name='home')
Including another URLconf
    1. Import the include() function: from django.conf.urls import url, include
    2. Add a URL to urlpatterns:  url(r'^blog/', include('blog.urls'))
"""
from django.conf.urls import url
import server.interface.routes as routeview

urlpatterns = [
    url(r'^route/generate$', routeview.route_from_coord),
    url(r'^route/return$', routeview.go_home),
    url(r'^route/rod$', routeview.rod),
    url(r'^route/convert$', routeview.convert),
    url(r'^route/parse$', routeview.parse),
    url(r'^route/rate$', routeview.rate),
    url(r'^route/import$', routeview.import_json),
    # get user statistics based on given userid

    # uppdate user statistics based on given userid, if id not in database --> add id
    # to table and put in the statistics

    # request route ----> return nodes with an id for the route --> with this id
    # the edges that were used can be found in the database and their ratings get updated.

    # send route id and rating to server, server processes and updates database if needed


    ]
