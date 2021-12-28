# Webhook
Based on information of the proximity-eu go example [repo](https://github.com/proxity-eu/WebhookBackendExample).

URL: https://scad-webhook-vfj4g5oy5q-oa.a.run.app 

Endpoints:
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
