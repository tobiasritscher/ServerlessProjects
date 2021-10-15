# SCAD-CLUELESS Group 04
SCAD ZHAW

## Team:
- Adrian Hornung (hornuadr)
- Jari Rentsch (rentsjar)
- Kunsang Kündetsang (kuendkun)
- Sydney Nguyen (nguyesyd)
- Tobi Ritscher (ritsctob)


## [P01:](/Labs/Lab01)
To achieve the tasks in this lab, we have created an account on the Google Cloud Platform. We used Python to implement a cloud function on this Faas service.
[Here](/Labs/Lab01/Group04_P01_Submission.pdf) you can find the report PDF of the Lab.

- [E0](/Labs/Lab01/E0/) / [function](https://europe-west6-formal-airway-260.cloudfunctions.net/ex1-e0)
- [E2](/Labs/Lab01/E2/) / [function](https://europe-west6-formal-airway-260.cloudfunctions.net/ex1-e2)
- [E4](/Labs/Lab01/E4/) / [function](https://europe-west6-formal-airway-260.cloudfunctions.net/ex1-e4)
- [E7](/Labs/Lab01/E7/) / [function](https://europe-west6-formal-airway-260.cloudfunctions.net/ex1-e7)
- [E8](/Labs/Lab01/E8/) / [function](https://europe-west6-formal-airway-260.cloudfunctions.net/ex1-E8)


## [P02:](/Labs/Lab02)
- [Frontend](/Labs/Lab02/frontend/) / [function](https://europe-west6-formal-airway-260.cloudfunctions.net/ex2-front)
- [Overview](/Labs/Lab02/overview/) / [function](https://europe-west6-daring-runway-326914.cloudfunctions.net/display-state-info) 
- [Analytics]() / [function](https://europe-west6-nomadic-line-328315.cloudfunctions.net/statisticsJava/?password=admin) 
  | user  | password |
  |-------|----------|
  | admin | admin    |


 ### Documentation:
  
  Our frontend and the overview page (displays all feedbacks) is written in Python and deployed on Google Cloud Functions. 
  Our analytics page is written in Java and is also deployed on Google Cloud Functions.
  We first intended to chose Amazon Web Services as the provider for the database but then we realised that it wasn't possible to get a free database with them. In the end we settled for a Firebase (Firestore) database since that was free and easy to use. 
  Since both GCF and Firebase are developed by Google we decided to make the analytics page in Java so that we have a polyglot application.  
  
  
## [P03:](/Labs/Lab03)
Our workflow manager and the dummy functions are listed below and the observations we have made are [here](https://github.zhaw.ch/nguyesyd/SCAD-CLUELESS/blob/master/Labs/Lab03/observations.txt)
- [manager](/Labs/Lab03/workflow-manager) / [function]()
- [Dummy00](/Labs/Lab03/Dummyfunctions/Dummy00.py) / [function](https://europe-west6-daring-runway-326914.cloudfunctions.net/dummy-function-0)
- [Dummy01](/Labs/Lab03/Dummyfunctions/Dummy01.py) / [function](https://europe-west6-daring-runway-326914.cloudfunctions.net/dummy-function-1)
- [Dummy02](/Labs/Lab03/Dummyfunctions/Dummy02.py) / [function](https://europe-west6-daring-runway-326914.cloudfunctions.net/dummy-function-2)
- [Dummy03](/Labs/Lab03/Dummyfunctions/Dummy03.py/) / [function](https://europe-west6-daring-runway-326914.cloudfunctions.net/dummy-function-3)
- [Dummy04](/Labs/Lab03/Dummyfunctions/Dummy04.py) / [function](https://europe-west6-daring-runway-326914.cloudfunctions.net/dummy-function-4)

### Documentation
- Dummy00: receives a String, returns a String (calls sleep function)
- Dummy01: receives a number, returns a number (fibonacci) 
- Dummy02: receives a String, returns a String (reverses the String)
- Dummy03: receives a String, returns a String (calls sleep function)
- Dummy04: receives a String, returns a String (does some multiplication)

### Example
```python
template_or = {
      "description": "f0-f1-f2|f3-f4",
      "ping": False,
      "urls":  [
          "https://europe-west6-daring-runway-326914.cloudfunctions.net/dummy-function-0", 
          "https://europe-west6-daring-runway-326914.cloudfunctions.net/dummy-function-1",
          "https://europe-west6-daring-runway-326914.cloudfunctions.net/dummy-function-2",
          "https://europe-west6-daring-runway-326914.cloudfunctions.net/dummy-function-3",
          "https://europe-west6-daring-runway-326914.cloudfunctions.net/dummy-function-4",
          ],
      "names": [
          "f0", 
          "f1", 
          "f2", 
          "f3", 
          "f4", 
      ]
}
=> 
result = {'tasks': [{'end': '2021-10-16T00:37:30.787012',
            'json': {'start received': 1},
            'name': 'f0',
            'start': '2021-10-16T00:37:29.527683',
            'status': 200,
            'urls': ['https://europe-west6-daring-runway-326914.cloudfunctions.net/dummy-function-0']},
           {'end': '2021-10-16T00:37:30.808242',
            'json': {'function 1': 0},
            'name': 'f1',
            'start': '2021-10-16T00:37:30.787813',
            'status': 200,
            'urls': ['https://europe-west6-daring-runway-326914.cloudfunctions.net/dummy-function-1']},
           {'end': '2021-10-16T00:37:30.828289',
            'json': {'function 2': 'ollaHillaH'},
            'name': 'f2',
            'start': '2021-10-16T00:37:30.808635',
            'status': 200,
            'urls': ['https://europe-west6-daring-runway-326914.cloudfunctions.net/dummy-function-2']},
           {'end': '2021-10-16T00:37:30.851052',
            'json': {'text': 'Function 4 has used some multiplication.'},
            'name': 'f4',
            'start': '2021-10-16T00:37:30.828679',
            'status': 200,
            'urls': ['https://europe-west6-daring-runway-326914.cloudfunctions.net/dummy-function-4']}]}
```
```python
template_and = {
      "description": "f0-f1-f2&f3-f4",
      "ping": False,
      "urls":  [
          "https://europe-west6-daring-runway-326914.cloudfunctions.net/dummy-function-0", 
          "https://europe-west6-daring-runway-326914.cloudfunctions.net/dummy-function-1",
          "https://europe-west6-daring-runway-326914.cloudfunctions.net/dummy-function-2",
          "https://europe-west6-daring-runway-326914.cloudfunctions.net/dummy-function-3",
          "https://europe-west6-daring-runway-326914.cloudfunctions.net/dummy-function-4",
          ],
      "names": [
          "f0", 
          "f1", 
          "f2", 
          "f3", 
          "f4", 
      ]
}
=> 
result = {'tasks': [{'end': '2021-10-16T00:32:20.149437',
            'json': {'start received': 1},
            'name': 'f0',
            'start': '2021-10-16T00:32:19.076021',
            'status': 200,
            'urls': ['https://europe-west6-daring-runway-326914.cloudfunctions.net/dummy-function-0']},
           {'end': '2021-10-16T00:32:20.174149',
            'json': {'function 1': 0},
            'name': 'f1',
            'start': '2021-10-16T00:32:20.150183',
            'status': 200,
            'urls': ['https://europe-west6-daring-runway-326914.cloudfunctions.net/dummy-function-1']},
           {'end': '2021-10-16T00:32:20.507381',
            'json': {'function 2': 'ollaHillaH',
                     'text': 'Function 3 has returned nicely after some '
                             'sleep.'},
            'name': 'f2&f3',
            'start': '2021-10-16T00:32:20.175688',
            'status': 200,
            'urls': ['https://europe-west6-daring-runway-326914.cloudfunctions.net/dummy-function-2',
                     'https://europe-west6-daring-runway-326914.cloudfunctions.net/dummy-function-3']},
           {'end': '2021-10-16T00:32:20.526016',
            'json': {'text': 'Function 4 has used some multiplication.'},
            'name': 'f4',
            'start': '2021-10-16T00:32:20.507767',
            'status': 200,
            'urls': ['https://europe-west6-daring-runway-326914.cloudfunctions.net/dummy-function-4']}]}
```
 
