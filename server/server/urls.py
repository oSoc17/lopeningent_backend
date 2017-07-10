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
import server.interface.nodes as nodeview
import server.interface.closest as closestview
import server.interface.routes as routeview
from server.interface.readme import readme

urlpatterns = [
    url(r'^node$', nodeview.get_node),
    url(r'^node/get-id$', closestview.get_id_from_pos),
    url(r'^node/get-node$', closestview.get_node_from_pos),
    url(r'^node/in-city$', nodeview.in_city),
    #url(r'^route/pathing$', routeview.distance),
    url(r'^route/generate$', routeview.route_from_coord),
    url(r'^route/return$', routeview.go_home),
    url(r'^route/rod$', routeview.rod),
    url(r'^route/convert$', routeview.convert),
    url(r'^README$', readme),
    url(r'^route/parse$', routeview.parse),
    url(r'^route/rate$', routeview.rate),
    url(r'^route/import$', routeview.import_json),
]
