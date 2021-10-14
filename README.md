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
Our workflow manager is written in Python and can be found [here]().
The dummy functions which are called by the manager are listed below:
- [f0](/Labs/Lab03/f0/) / [function](https://europe-west6-formal-airway-260.cloudfunctions.net/ex1-e0)
- [f1](/Labs/Lab03/f1/) / [function](https://europe-west6-formal-airway-260.cloudfunctions.net/ex1-e2)
- [f2](/Labs/Lab03/f2/) / [function](https://europe-west6-formal-airway-260.cloudfunctions.net/ex1-e4)
- [f3](/Labs/Lab03/f3/) / [function](https://europe-west6-formal-airway-260.cloudfunctions.net/ex1-e7)
- [f4](/Labs/Lab03/f4/) / [function](https://europe-west6-formal-airway-260.cloudfunctions.net/ex1-E8)

### Documentation:
 The observation we made from this experiment can be found [here](https://github.zhaw.ch/nguyesyd/SCAD-CLUELESS/blob/master/Labs/Lab03/observations.txt).


 
