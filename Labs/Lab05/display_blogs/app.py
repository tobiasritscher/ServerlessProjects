import os
import requests
import flask

app = flask.Flask(__name__)

@app.route('/')
def hello_world():    
    r = requests.get('https://scaddb.ritscher.ch/get')
    myJson = r.json()["all"]
    return flask.render_template("form_submitted.html", title="Blog Posts", blogs=myJson)

    
if __name__ == "__main__":
    app.run(debug=True, host='0.0.0.0',port=int(os.environ.get('Port', 8080))) 