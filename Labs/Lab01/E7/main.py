from bs4 import BeautifulSoup
import requests

def parse_webpage():
    url = "https://www.zhaw.ch/de/studium/bachelorstudiengaenge/"
    names=[]
    # content of URL
    r = requests.get(url)
      
    # Parse HTML Code
    soup = BeautifulSoup(r.text, 'html.parser')
      
    # find all images in URL
    titles = soup.findAll('h3')

    return [title.get_text().strip() for title in titles]  


def hello_world(request):
    names = parse_webpage()
    return { "Studiengänge": names }

