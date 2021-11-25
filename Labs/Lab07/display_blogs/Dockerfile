From python:3.7-alpine

ENV APP_HOME /app
ENV PATH "$PATH:/home/iuser/.local/bin"
WORKDIR $APP_HOME
COPY . ./

RUN addgroup -S iuser && adduser -S iuser -G iuser 
#RUN adduser --system --group iuser

USER iuser

RUN pip install --no-cache-dir -r requirements.txt

EXPOSE 8000

CMD exec gunicorn --bind 0.0.0.0:8000 --workers 2 --threads 2 --timeout 0 app:app