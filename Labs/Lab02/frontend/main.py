import os
import logging
import datetime 

import flask 
import flask_wtf 
import wtforms 
from wtforms.csrf.session import SessionCSRF

import firebase_admin
import firebase_admin.firestore

class Config:
    PROJECT_ID          = os.environ.get("PROJECT_ID", default="")
    PRIVATE_KEY         = os.environ.get("PRIVATE_KEY", default="")
    PRIVATE_KEY_ID      = os.environ.get("PRIVATE_KEY_ID", default="")
    CLIENT_EMAIL        = os.environ.get("CLIENT_EMAIL", default="")
    CLIENT_ID           = os.environ.get("CLIENT_ID", default="")
    CLIENT_CERT_URL     = os.environ.get("CLIENT_CERT_URL", default="")
    DATABASE_COLLECTION = "feedbacks"

    CSRF_SECRET = os.environ.get("CSRF_SECRET", default="")
    
    @classmethod
    def not_setup(cls):
        values = []

        for val in dir(cls):
            inner = getattr(cls, val) 
            if callable(inner) or val.startswith("_"):
                continue
            if inner is None or inner == "":
                values.append(val)
        return values


class FrontendForm(flask_wtf.FlaskForm):
    text = wtforms.TextAreaField('Text', validators=[wtforms.validators.DataRequired()])
    submit = wtforms.SubmitField('Submit')

    class Meta:
        csrf = False
        csrf_class = SessionCSRF
        csrf_secret = Config.CSRF_SECRET 
        csrf_time_limit = datetime.timedelta(minutes=20)

class Results:
    def __init__(self, text):
        self.text = text
        self.datetime = datetime.datetime.now(datetime.timezone.utc).isoformat()

    def to_obj(self):
        return {
                "feedback_text": self.text,
                "timestamp": self.datetime
        }

def handle_invalid_form(request, form):
    return handle_get(request, form)

def handle_post(request, form):
    if form.validate_on_submit():
        res = Results(form.text.data)
        db_client = setup_database_conn()
        db_client.collection(Config.DATABASE_COLLECTION).add(res.to_obj())
        return flask.render_template("form_submitted.html", title="Sucess submitting", result=res)
    else:
        return handle_invalid_form(request,form)

def handle_get(request, form):
    return flask.render_template("form.html", title="Form", form=form)

_DB_CLIENT = None
def setup_database_conn():
    global _DB_CLIENT
    if not _DB_CLIENT:
        certs = {
            "type": "service_account",
            "project_id": Config.PROJECT_ID,
            "private_key": Config.PRIVATE_KEY.replace("\\n", "\n"),
            "private_key_id": Config.PRIVATE_KEY_ID,
            "client_email": Config.CLIENT_EMAIL,
            "client_id": Config.CLIENT_ID,
            "auth_uri": "https://accounts.google.com/o/oauth2/auth",
            "token_uri": "https://oauth2.googleapis.com/token",
            "auth_provider_x509_cert_url": "https://www.googleapis.com/oauth2/v1/certs",
            "client_x509_cert_url": "https://www.googleapis.com/robot/v1/metadata/x509/firebase-adminsdk-myllf%40feedback-form-6f821.iam.gserviceaccount.com",
        } 

        creds = firebase_admin.credentials.Certificate(certs)
        firebase_admin.initialize_app(creds)
        _DB_CLIENT = firebase_admin.firestore.client()
    return _DB_CLIENT

def main(request):
    # check if function is setup correctly
    if len(conf := Config.not_setup()) > 0:
        logging.error(f"missing database configuraion for {conf}")
        flask.abort(500)

    form = FrontendForm()
    if request.method == "GET":
        return handle_get(request, form)
    elif request.method == "POST":
        return handle_post(request, form)
    else:
        flask.abort(405)

