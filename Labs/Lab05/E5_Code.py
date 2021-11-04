from textblob import TextBlob

def detect_language(text):
    b = TextBlob(text)
    return b.detect_language()
