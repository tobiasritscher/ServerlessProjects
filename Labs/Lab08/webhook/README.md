# Webhook
Based on information of the proximity-eu go example [repo](https://github.com/proxity-eu/WebhookBackendExample).

The container is hosted on [Docker Hub](https://hub.docker.com/repository/docker/thebluefirefox/scad-webhook) and 
only need a few enviroment variables to setup correctly. 

URL: https://scad-webhook-vfj4g5oy5q-oa.a.run.app 

### ENV-variables:
- `ADDRESS` is the ip address the server will be listening on 
   and should **not be changed**, as otherwise the server will not be able to communicate.<br>
   defaults to `0.0.0.0`
- `PORT` is the port address that the server is listening on and will be set by 
   the google cloud run runtime. <br>
   defaults to `8000`
- `DB` is the correct link to the database function<br>
   If this is not set there will be a warning log, however the server will still 
   run. 
- `LOG` is the dynamic logging level that is supported by the system<br>
   Supported levels:  
    - `error`
    - `warn` (default)
    - `info`
    - `debug`
    - `trace`



### Quality Analysis
#### Dockerfile:
- The end stage uses an artificial `unprivileged user`. To not have the server run as root and with that 
  expose to a successful hacker root capabilities.
- The docker container exposes inbuild `healthcheck` capabilities. 
- It also uses `scratch` as the image base, so that the resulting build size is as small as possible. (2.36MB)

#### Endpoints
All endpoints check that only the correct HTTP Content-Type is used according to the [spec](https://github.com/proxity-eu/WebhookBackendExample).


### Endpoints:
POST requests:
- [/webhook](https://scad-webhook-vfj4g5oy5q-oa.a.run.app/webhook) Gets new information from the 
Protxity app and will forward it to the DB.
It only accepts JSON formated data, with the either the 'text/plain' and 'application/json' ContentType header set.
Additionally `region_id` and the `device_data` field are optional and will be added if needed with an empty string 
before sending it to the DB.

```
{
    "id":165623652695468451651,
    "data":"Some more data for another datapoint",
    "region_id": 597956666487987,
    "device_data":"test devide data",
    "timestamp":"2021-12-28T12:30:39.615678876Z"
}
```

GET requests:
- [/ (root)](https://scad-webhook-vfj4g5oy5q-oa.a.run.app/) Root gives an overview over all the endpoints
- [/stats](https://scad-webhook-vfj4g5oy5q-oa.a.run.app/stats) Will return the webhook request 
information send during the last five minutes. Attention due to scaling this information is not safe to use
