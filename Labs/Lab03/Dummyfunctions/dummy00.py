import time
def hello_world(request):
  request_json = request.get_json()
  if request_json and 'start' in request_json:
    time.sleep(1)
    return {"start received":1}
  return {}