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
Our workflow manager and the dummy functions are listed below and the observations we have made are [here](https://github.zhaw.ch/nguyesyd/SCAD-CLUELESS/blob/master/Labs/Lab03/observation.md)

- [Manager](/Labs/Lab03/workflow-manager) / [function](https://europe-west6-formal-airway-260.cloudfunctions.net/ex3-manager)
- [Dummy00](/Labs/Lab03/Dummyfunctions/Dummy00.py) / [function](https://europe-west6-daring-runway-326914.cloudfunctions.net/dummy-function-0)
- [Dummy01](/Labs/Lab03/Dummyfunctions/Dummy01.py) / [function](https://europe-west6-daring-runway-326914.cloudfunctions.net/dummy-function-1)
- [Dummy02](/Labs/Lab03/Dummyfunctions/Dummy02.py) / [function](https://europe-west6-daring-runway-326914.cloudfunctions.net/dummy-function-2)
- [Dummy03](/Labs/Lab03/Dummyfunctions/Dummy03.py/) / [function](https://europe-west6-daring-runway-326914.cloudfunctions.net/dummy-function-3)
- [Dummy04](/Labs/Lab03/Dummyfunctions/Dummy04.py) / [function](https://europe-west6-daring-runway-326914.cloudfunctions.net/dummy-function-4)

### Documentation
- Manager: receives a json directory with fields such as shown below
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
 
 
 
## [P04:](/Labs/Lab04)

### R2:
Created a basic [workflow](https://github.zhaw.ch/nguyesyd/SCAD-CLUELESS/tree/master/Labs/Lab04/workflow) with [Google Cloud Workflows](https://cloud.google.com/workflows). The first step in the Workflow is to call the ping function, then Frontend, Overview and Analytics.

This is the output after calling the workflow:

```
Waiting for execution [d7d92380-2d6d-4a4f-8a34-bc10ff566f44] to complete...done.     
argument: 'null'
endTime: '2021-10-28T15:07:32.048832311Z'
name: projects/192660774501/locations/us-central1/workflows/workflow-sa/executions/d7d92380-2d6d-4a4f-8a34-bc10ff566f44
result: 'null'
startTime: '2021-10-28T15:07:26.908865616Z'
state: SUCCEEDED
workflowRevisionId: 000001-ac2
```
### R5:
We tried to implement a function through faasification of python native code, that detects the language of a given feedback. Unfortunately the faasification tool Lambada had an bug and was unable to deploy the function to GCF. Here you can find the [function](https://github.zhaw.ch/nguyesyd/SCAD-CLUELESS/blob/master/Labs/Lab04/faasification/R5_Code.py) and the [error message](https://github.zhaw.ch/nguyesyd/SCAD-CLUELESS/blob/master/Labs/Lab04/faasification/R5_Errortext).

### R6:
This [cloud function](https://europe-west6-mesmerizing-app-326913.cloudfunctions.net/r6) enables warm-up pinging which we need for self-optimization. 

### R7:
This [function](https://europe-west6-formal-airway-260.cloudfunctions.net/ex4-trigger) is a content trigger
 
### R8:
We put OpenFaas on a [raspberry pie](https://faas.ritscher.ch) and used a selfhosted [docker registry](https://dockerhub.ritscher.ch) to deploy the docker container with the cloud function.

## [P05:](/Labs/Lab05)
- [Display_Blogs](https://github.zhaw.ch/nguyesyd/SCAD-CLUELESS/tree/master/Labs/Lab05/display_blogs)/[link](https://hello-python-yiw4r3uyta-oa.a.run.app/): We made a short-lived container that runs in the GCP, that displays a list of our blog posts. It's a flask server that returns an HTML page with the list of blogs.
- [Database](https://github.zhaw.ch/nguyesyd/SCAD-CLUELESS/tree/master/Labs/Lab05/long-rocket/)/[link](https://scaddb.ritscher.ch/): This is one of the long living crates. It is written primarily in rust but uses some c dependencies, so that we satisfy the given poligot requirements. Additionally to enable a persistent data state we host it on our own service with a dedicated docker volumn mounted in at all time. 
- [Make a new Blog](https://scadlab5.ritscher.ch/)/[link](https://github.zhaw.ch/nguyesyd/SCAD-CLUELESS/tree/master/Labs/Lab05/submit): Here you can write a blog which will be saved in the database. This long living container runs on our privatly hosted server.

### Project explanation
In this project we implemented a simple blog platform, where everyone can post a new message and this will be shown in the blog. In the backround the messages will be saved in a databse with a timetag. The complexity is in the connection between the containers with different languages and different locations as well as saving all data in a sqlite file so the data won't be lost when the server or the container is restartet.

This project uses Google Cloud as well as a private hosted cloud server with Unraid OS. There is also a selfhosted dockerhub (https://dockerhub.ritscher.ch) running inside a container, but we can't share the credentials because of security reasons. We used Rust and Python as the programming language.

### Dockerhub links:
Make a new Post: https://hub.docker.com/r/thebluefirefox/scad-ex5-submit
Database: https://hub.docker.com/r/thebluefirefox/scad-long-rocket-db 


## [P06:](/Labs/Lab06)

### Project explanation
In this lab we used [Ballerina](https://ballerina.io) to develope [three micro services](/Labs/Lab06/myServices). The first one, called service1, takes an integer as a URL parameter and invokes service2 with the integer as parameter.
Ladder one performs a multiplication with specified constant and invokes service3 with the result again as a parameter. Finally, service3 does the same task as service2 but with another specified constanz.
The [raml file](/Labs/Lab06/api.raml) makes the micro services discoverable. 

## [P07:](/Labs/Lab07)
### Quality Analysis
#### Weaknesses 
1) [Dockerfile  long-rocket](/Labs/Lab07/long-rocket/Dockerfile): new smaller base image<br>
Using different a base image as the previous one was on the bigger side. A bigger image will automatically use up more ram in the host system which might cost more. Additionally on scalable systems, where the docker container has to be copied from the storage system to different hosts for load balancing. A smaller image will yield significantly faster start up time.<br>
By changing some of the application parameters to make it compile with [musl](https://musl.libc.org/) instead of the default [glibc](https://en.wikipedia.org/wiki/Glibc) libraries and using static linking of dynamic instead. We were able to switch to the apline:latest base image from the debian:buster-slim. By applying all these steps we were able to go from a 100Mb large container image to a 11Mb large one.
2) [Dockerfile  long-rocket](/Labs/Lab07/long-rocket/Dockerfile): healthcheck<br>
A healthcheck will help on long running systems as the administrator gets to check the containers availability without needing to call the application output, assuming there even is an output in the first place. <br>
Adding a line to the Dockerfile we got the healthcheck working. 
3) [Dockerfile Display-Blogs](/Labs/Lab07/display_blogs/Dockerfile): switch to smaller base image<br>
We changed the base image from Python:3.7-slim to Python:3.7-alpine, which reduced it's size by 87 MB.
4) [Dockerfile Display-Blogs](/Labs/Lab07/display_blogs/Dockerfile) removal of unnecessary packages and files <br>
By merging the two previous containers (including the functionality) from [Lab5](/Labs/Lab5), we were able to reduce the number of files and libraries necessary to offer the same funtionality. The new [container](/Labs/Lab07/display_blogs) contains the functionality of these to containers ([display_blogs](/Labs/Lab05/display_blogs) and [submit](/Labs/Lab05/submit)).
