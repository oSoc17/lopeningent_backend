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
import interface.stats as stats
import interface.pois as pois

urlpatterns = [
    url(r'^stats/check/', stats.get_stats_from_id ),
    url(r'^stats/update/', stats.post_stats_from_id),
    # url(r'^route/generate/', route.generate),
    # url(r'^route/return/', route.return_home),
    # url(r'^route/rate/', route.rate_route),
    url(r'^poi/coords/', pois.get_coords),
    url(r'^poi/types/', pois.get_types)
]
