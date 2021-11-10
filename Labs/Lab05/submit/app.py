import os
import logging
import datetime
import requests
import flask
import flask_wtf
import wtforms
from wtforms.csrf.session import SessionCSRF


# BASE application
app = flask.Flask(__name__)


class Config:

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
    text = wtforms.TextAreaField(
        'Text', validators=[wtforms.validators.DataRequired()])
    submit = wtforms.SubmitField('Submit')

    class Meta:
        csrf = False
        csrf_class = SessionCSRF
        csrf_secret = Config.CSRF_SECRET
        csrf_time_limit = datetime.timedelta(minutes=20)


class Results:
    def __init__(self, text):
        self.text = text
        self.datetime = datetime.datetime.now(
            datetime.timezone.utc
        ).isoformat()

    def to_obj(self):
        return {
            "text": self.text,
            "timestamp": self.datetime
        }


def handle_invalid_form(request, form):
    return handle_get(request, form)


def handle_post(request, form):
    if form.validate_on_submit():
        res = Results(form.text.data)
        resp = requests.post(
            "https://scaddb.ritscher.ch/set", json=res.to_obj())

        return flask.render_template("form_submitted.html", title="Sucess submitting", result=res)
    else:
        return handle_invalid_form(request, form)


def handle_get(request, form):
    return flask.render_template("form.html", title="Form", form=form)


@app.route("/", methods=['GET', 'POST'])
def main():
    request = flask.request

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


if __name__ == "__main__":
    app.run(debug=True, host='0.0.0.0', port=int(os.environ.get('Port', 8080)))
