import os
import requests
import flask
import datetime
app = flask.Flask(__name__)
#baseAdress = 'https://scaddb.ritscher.ch'
baseAdress = os.environ.get("DB_IP")

@app.route('/', methods=['GET','POST'])
def main():  
    
    if flask.request.method == "POST":
        r = requests.post('https://'+baseAdress+'/set', json={
            "text": flask.request.form["send_blog"],
            "timestamp" : datetime.datetime.now(datetime.timezone.utc ).isoformat()
        })
        r.raise_for_status()

    r = requests.get('https://'+baseAdress+'/get')
    r.raise_for_status()
    myJson = r.json()["all"]
    return flask.render_template("index.html", title="Our Special Blog", blogs=myJson)

    
if __name__ == "__main__":
    app.run(debug=True, host='0.0.0.0',port=int(os.environ.get('Port', 8000))) 
