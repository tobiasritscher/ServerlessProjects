def hello_world(request): 
    if (request != {}):
        time.sleep(2)
        return "Function 3 has returned nicely after some sleep."
    return ""

#print(hello_world({"hihihihi":"hi"}))
#print(hello_world({}))
