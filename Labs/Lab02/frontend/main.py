import os
from flask import abort, render_template
import logging

DEBUG = True

class Config:
    DATABASE_PWD  = os.environ.get("DATABASE_PWD", default=None)
    DATABASE_USER = os.environ.get("DATABASE_USER", default=None)
    DATABASE_NAME = os.environ.get("DATABASE_NAME", default=None)

    @classmethod
    def is_setup(cls):
        values = [
                cls.DATABASE_PWD,  
                cls.DATABASE_USER, 
                cls.DATABASE_NAME 
        ]
        for val in values:
            if val is None:
                return False
        return True


def main(request):
    # make the IDE shutup
    _ = request
    # check if function is setup correctly
    if not Config.is_setup():
        logging.error("missing database configuraion")
        if not DEBUG:
            abort(500)

    user = {'username': "Adrian"}

    return render_template("index.html", title="Form", user=user)
