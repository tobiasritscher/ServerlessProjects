def hello_world(request):
  request_json = request.get_json()
  if request.args and '' in request.args:
    return ''
  elif request_json and 'message' in request_json:
    return dummy(request_json['message'])
  else:
    return ''

def dummy(nString):
  n = len(nString)**2
  for i in range(n):
    print(recur_fibo(i))


def recur_fibo(n):
  if n <= 1:
    return n
  else:
    return(recur_fibo(n-1) + recur_fibo(n-2))


