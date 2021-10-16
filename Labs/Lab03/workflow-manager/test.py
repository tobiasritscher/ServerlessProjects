from re import template
import requests
import pprint

template_or = {
      "description": "f0-f1-f2|f3-f4",
      "ping": True,
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

template_and = {
      "description": "f0-f1-f2&f3-f4",
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

template = template_or

# other = "http://172.22.224.36:8080/" 
other = "https://europe-west6-formal-airway-260.cloudfunctions.net/ex3-manager"

def main():
    print("online making request")
    r = requests.post(other, json=template) 
    print("got request")
    print(r.status_code)
    data = r.json()
    pp = pprint.PrettyPrinter()
    pp.pprint(data)


if __name__ == "__main__":
    main()
