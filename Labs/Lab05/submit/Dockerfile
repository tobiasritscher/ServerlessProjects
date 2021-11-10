FROM python:3.9-slim

ENV CSRF_SECRET="EPj00jpfj8Gx1SjnyLxwBBSQfnQ9DJYe0Ym"
WORKDIR /app

# Preloading the dependencies, so that they don't have to
# be downloaded every single time
COPY requirements.txt .

RUN pip install --no-cache-dir -r requirements.txt

# Getting the rest of the application
COPY . .

EXPOSE 8000

CMD ["sh", "./run.sh"]

