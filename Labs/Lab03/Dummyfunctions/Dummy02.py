def hello_world(request):
  request_json = request.get_json()
  if request.args and '' in request.args:
    return ''
  elif request_json and 'message' in request_json:
    return dummy(request_json['message'])
  else:
    return ''

def dummy(nString):
  reversedString = ''
  for letter in nString:
    reversedString = reversedString + letter
  return reversedString
