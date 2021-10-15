def hello_world(request):
  request_json = request.get_json()
  if request_json:
    return {"function 1":dummy(request_json)}
  return {}

def dummy(nString):
  n = len(nString)**2
  temp = ''
  for i in range(n):
    print(recur_fibo(i))
    temp = recur_fibo(i)
  return temp


def recur_fibo(n):
  if n <= 1:
    return n
  else:
    return(recur_fibo(n-1) + recur_fibo(n-2))
