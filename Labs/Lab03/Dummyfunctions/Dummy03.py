def hello_world(request): 
    if (request != {}):
        time.sleep(0.2)
        return {"text":"Function 3 has returned nicely after some sleep."}
    return {}

#print(hello_world({"hihihihi":"hi"}))
#print(hello_world({}))
