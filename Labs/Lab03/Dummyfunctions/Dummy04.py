def hello_world(request): 
    request_json = request.get_json()
    if request_json:
        a = 1
        b = 100
        c = 123
        d = a*b
        d = d*c
        return {"text":"Function 4 has used some multiplication."}
    return {}

#print(hello_world({"hihihihi":"hi"}))
#print(hello_world({}))
