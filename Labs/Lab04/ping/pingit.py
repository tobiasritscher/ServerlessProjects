def pingit(request):
    url = "https://europe-west6-daring-runway-326914.cloudfunctions.net/dummy-function-1"
    param = {}
    response = request.post(url, json=param)
    return {
        "json": response.json(),
        "status": response.status
    }
