import os
import logging
import datetime 

import flask 
import flask_wtf 
import wtforms 
from wtforms.csrf.session import SessionCSRF

DEBUG = True

class Config:
    DATABASE_PWD  = os.environ.get("DATABASE_PWD", default=None)
    DATABASE_USER = os.environ.get("DATABASE_USER", default=None)
    DATABASE_NAME = os.environ.get("DATABASE_NAME", default=None)
    # TODO: remove deault and replace with None
    CSRF_SECRET = os.environ.get("CSRF_SECRET", default="EPj00jpfj8Gx1SjnyLxwBBSQfnQ9DJYe0Ym").encode("utf8")

    @classmethod
    def not_setup(cls):
        values = []

        for val in dir(cls):
            if callable(getattr(cls, val)) or val.startswith("_"):
                continue
            if getattr(cls, val) is None:
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

def handle_invalid_form(request, form):
    return handle_get(request, form)

def handle_post(request, form):
    # TODO: handle parsing of the form
    if form.validate_on_submit():
        res = Results(form.text.data)
        return flask.render_template("form_submitted.html", title="Sucess submitting", result=res)
    else:
        return handle_invalid_form(request,form)

def handle_get(request, form):
    return flask.render_template("form.html", title="Form", form=form)

def main(request):
    # check if function is setup correctly
    if len(conf := Config.not_setup()) > 0:
        logging.error(f"missing database configuraion for {conf}")

        if not DEBUG:
            flask.abort(500)

    form = FrontendForm()
    if request.method == "GET":
        return handle_get(request, form)
    elif request.method == "POST":
        return handle_post(request, form)
    else:
        flask.abort(405)

