#!/bin/sh
gunicorn --bind 0.0.0.0:8000 --workers 2 --threads 2 --timeout 0 app:app

