from re import template
import requests
import pprint

template_or = {
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

url = "http://192.168.0.243:8080/"
url = "https://europe-west6-formal-airway-260.cloudfunctions.net/ex3-manager"

def req(template, name):
    print(f"running {name}")
    r = requests.post(url, json=template)
    print("got request")
    print(r.status_code)
    return r.json()


def main():
    print("online")
    pp = pprint.PrettyPrinter()
    todo = [("OR", template_or), ("AND", template_and)]
    for name, template in todo:
        data = req(template, name)
        pp.pprint(data)

if __name__ == "__main__":
    main()
