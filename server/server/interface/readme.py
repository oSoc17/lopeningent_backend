from django.http import HttpResponse, HttpResponseNotFound

def readme(request):
    try:
        with open("README") as readme:
            return HttpResponse(readme.read())
    except BaseException:
        return HttpResponseNotFound()
