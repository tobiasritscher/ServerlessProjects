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
    
    count = 0
    limit = 10
    for title in titles:
        count += 1
        name = title.get_text().strip()
        names.append(name)
        if count >= limit:
            break
    return names  


def hello_world(request):
    names = parse_webpage()
    return ",".join(names)