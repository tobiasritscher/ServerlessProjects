# ProLoc
This project defines and implements a prototype for the ProLoc application. ProLoc will help visually impaired people to naviagte the city. The app is pretty simple and it should be as lightweight as possible, so the users aren't overwhelmed by the features but rather have a ready to use experience. 
While running, the app is always looking out for beacons. If one is found, the ID of the beacon will be checked in our database and if it is a match, the name of the according building will be returned and showed to the user as a push notification. If the user has turned on the integrated operating aids of his smartphone, the push notification will be read to him by the voice assistance. In the app the user can also define if he is looking for something special, like a shopping center or the ZHAW, and the app will only notify him about relevant buildings. In the future it would also be possible to support public transportation.

# Diagram
This diagram shows the current setup, of our application. 
The system is build up from different parts all working over an HTTP event based system.

## Beacon Event
1) The [webhook](webhook) receives a beacon event send by Proxity.
   1) It will forward a now correcly formatted json string to the [databse coud function](cloud functions). 
2) The DB functions will insert the new data, after creating it a new random id for it, into the firestone db.

## Frontend Display Event
1) The Frontend will call the [databse coud function](cloud functions) to get the 
   required information.
2) DB function will querry the databse for the required information.
   1) And return the results

![architecture diagram](imgs/architecture_diagram.svg?raw=true "Architecture Diagram")
