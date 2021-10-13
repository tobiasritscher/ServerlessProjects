import firebase_admin
from firebase_admin import credentials
from firebase_admin import firestore
import datetime

# initializations 
jsonfile = {
  "type": "service_account",
  "project_id": "feedback-form-6f821",
  "private_key_id": "3eeef65647e91369b41db800ac64a494d1776e21",
  "private_key": "-----BEGIN PRIVATE KEY-----\nMIIEvgIBADANBgkqhkiG9w0BAQEFAASCBKgwggSkAgEAAoIBAQDk+SCUGz8WOHr+\nA9Hg+ZxU+SLqa4KB8GhP44Z3PGlUKbwaBJzVoy2q4J5+b9pbojQrfBR0n9u2bQtQ\nET2XUnR39NqIbDeJvtER8HvTz3BRgJBs1uRuNkaZxz5Iag8e+4+bwWqj+jtzjGwb\nl1w8k61o51D6O/TRxMeMLcar7YoFhWQckEqtTOoy1V7TCgowhBMcaRnFTJGnUKwB\nYw2xbQq2ic1Jh5y9f2JMBQzoYGoF1CTk8uy0VrI8WURg1mWFWNpCdbsFGU0MJSVB\nOUENqZrxjnmNoeat1ORX38tALJo27wJAzk08XUCTTbZ5FzzV9A9GShGfAwLzHIdh\nVwHAeVjNAgMBAAECggEACcIYQlea666Ch1rQUf8xS2nGtCjaygDd4TG4JFK9xml0\n68LwkCZBJvwlpQvNSYRzJrtF/+ZkLbUd+06QymcoMYiM262I/GTWciZGkLNRfRe2\nj/zkhVHo1lroa5uhg1eDOkcW7WAXsof0f4SG2MoOZN8e01tExr/VJUM6KU+ZRzzg\nbzI5ZEO95FMR1zsLh3DyLCy+8Ml+KiAZ8Hvrw36v0UQVvGQTlLuvheTOcZlUBnNM\n1An0esCAzDBdcNShdfySjrmS5nWHUUMLaw88wa7+ETobsKtYlZONYYZmkpC3FNzv\nMvaT3tS3MI/ul5tjRDsBOWFzKiiep8C5IXQnqTaJ4QKBgQD5R4rYCuZR7epfdNDR\nBy+hbcA1JTKHdIu3LZlWR8W70PhUVDiJSvd5e/m2Qoa0+XPHBk4HrSMCZVwYETjM\nqUFTdyOVMfAj8jDV9jwtQGwWwQrXmga5OKXZGjtgFinVnkY2ACPM4fPKXziucaFm\npK+chbDOQBDDTIhNQxZD2JxvWQKBgQDrJW+3hCQ4Y6B29C+fCk5WodXLdluUXIp5\nCSVCgtFxLlp6ShwGcw5wgTA+3xDT9RDIQNRTaS+DDCKx7hXTR+DZQkfTJYWSb3AT\ntLOVsB275bSV34ehmF4o7klkwE2X0Ho65JILRNGVP/yLaro5lZuStJ8US3FlGsQs\nuE2nsNmalQKBgQDOxRoVFA1x3g/vd/0RtY07sMajDqS4GOMfYVJ0b/pDTRX/FREE\nSMu32C8MkGgeS2BOsRseYO3OOfdG+1oK4UYPjp5UTeTsXCNIBwRpYKuHwD5XEisL\njkJ7L7gafqfayntgpYMMmkvXDF7+y8BeXLJR+c7rddaxuJTCyaf5RP6VmQKBgEvk\nqUnxvD726LN73GLYULBlSzt/187qj0bBTOVKk46UB1+wy5rJR7/H8mpYVHYUgnYf\ndmf1awBAynHjBehLKqb0XXooghPhI6s4hz2oufALE13L51qgcsODJOz/pAt7nfek\nDf7VF0+0dy4IJFKxlqSi5zUFpHOdT/OfCHE3AwSBAoGBAMygqbnx9dsn34iZH1eF\nQ/tEXIPmbkN5R81kzqmpY0bPouoQoHbUP+Yd/94EBn3jFH9hRL+uYw2d4zUc2mrm\nck/lsZDij5VQ759rPQAL4r1TOdkC+fLPxEV+RQ6lchTYVp5EUk9tjNEDjjxgpCZS\nHb4AwEwzVKPVcai4UDMDuHZf\n-----END PRIVATE KEY-----\n",
  "client_email": "firebase-adminsdk-myllf@feedback-form-6f821.iam.gserviceaccount.com",
  "client_id": "117998006178078434754",
  "auth_uri": "https://accounts.google.com/o/oauth2/auth",
  "token_uri": "https://oauth2.googleapis.com/token",
  "auth_provider_x509_cert_url": "https://www.googleapis.com/oauth2/v1/certs",
  "client_x509_cert_url": "https://www.googleapis.com/robot/v1/metadata/x509/firebase-adminsdk-myllf%40feedback-form-6f821.iam.gserviceaccount.com"
}

cred = credentials.Certificate(jsonfile)
firebase_admin.initialize_app(cred)
db = firestore.client()

#adding example data
def add(text="default"):
    db.collection(u'feedbacks').add({
        'feedback_text':text,
        'timestamp': datetime.datetime.now(datetime.timezone.utc).isoformat()
    })

#Reading the data
def read_db():
    emp_ref = db.collection('feedbacks')
    docs = emp_ref.stream()
    output = []
    for doc in docs:
        output.append(doc.to_dict())

    return output

def hello_world(request):
    #add('Lets do thissss.')
    return read_db()

print(hello_world(""))
