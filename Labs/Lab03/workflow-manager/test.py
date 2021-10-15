import requests

template = {
      "description": "f0-f1-f2|f3-f4",
      "ping": False,
      "urls":  [
          "https://europe-west6-daring-runway-326914.cloudfunctions.net/dummy-function-0", 
          "https://europe-west6-daring-runway-326914.cloudfunctions.net/dummy-function-1",
          "https://europe-west6-daring-runway-326914.cloudfunctions.net/dummy-function-2",
          "https://europe-west6-daring-runway-326914.cloudfunctions.net/dummy-function-3",
          "https://europe-west6-daring-runway-326914.cloudfunctions.net/dummy-function-4",
          ],
      "names": [
          "f0", 
          "f1", 
          "f2", 
          "f3", 
          "f4", 
      ]
}
other = "http://192.168.0.243:8080/" 

def main():
    print("online making request")
    r = requests.post(other, json=template) 
    print("got request")
    print(r.status_code)
    print(r.json())


if __name__ == "__main__":
    main()
