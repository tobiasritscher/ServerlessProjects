From python:3.7-slim

ENV APP_Home /app
WORKDIR $APP_Home
COPY . ./

RUN pip install Flask gunicorn requests

CMD exec gunicorn --bind :$PORT --workers 1 --threads 8 --timeout 0 app:app