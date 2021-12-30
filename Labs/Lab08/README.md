# Diagram
This diagram shows the current setup, of our application. 
The system is build up from different parts all working over an HTTP event based system.

## Beacon Event
1) The [webhook](webhook) receives a beacon event send by Proxity.
1.1) It will forward a now correcly formatted json string to the [databse coud function](cloud functions). 
2) The DB functions will insert the new data, after creating it a new random id for it, into the firestone db.

## Frontend Display Event
1) The Frontend will call the [databse coud function](cloud functions) to get the 
   required information.
2) DB function will querry the databse for the required information.

![architecture diagram](imgs/architecture_diagram.svg?raw=true "Architecture Diagram")
