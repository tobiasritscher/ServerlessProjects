def hello_world(request):
  request_json = request.get_json()
  if request_json:
    return {"function 2":dummy()}
  return {}

def dummy():
  nString="HalliHallo"
  reversedString = ''
  for letter in nString:
    reversedString = letter + reversedString
  return reversedString
