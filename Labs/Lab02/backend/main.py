import firebase_admin
from firebase_admin import credentials
from firebase_admin import firestore
import time
# Use the application default credentials
cred = credentials.ApplicationDefault()
firebase_admin.initialize_app(cred, {
  'projectId': 'feedback-form-6f821',
})

db = firestore.client()

def add_example():
  doc_ref = db.collection(u'feedbacks').document(u'firstfeedback')
  doc_ref.set({
      u'text': u'My first feedback text',
      u'timestamp': time.time()
  })

def read_print():
  feedbacks_ref = db.collection(u'feedbacks')
  docs = feedbacks_ref.stream()
  output = ""
  for doc in docs:
    output.append(f'{doc.id} => {doc.to_dict()}')
    print(f'{doc.id} => {doc.to_dict()}')
  return output

def hello_world(request):
  add_example()
  return read_print()
