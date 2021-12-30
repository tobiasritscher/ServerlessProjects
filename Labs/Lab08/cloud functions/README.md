URL: https://us-central1-beacon-2de55.cloudfunctions.net

Endpoints:


POST request:

- [/createDatapoint](https://us-central1-beacon-2de55.cloudfunctions.net/createDatapoint) creates a new datapoint

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

- [/readAllDatapoints](https://us-central1-beacon-2de55.cloudfunctions.net/readAllDatapoints) returns all datapoints

- [/readDatapoint/regionid/6f5b625d-2eb4-44e5-ae41-8eb9b8048f2c](https://us-central1-beacon-2de55.cloudfunctions.net/readDatapoint/regionid/6f5b625d-2eb4-44e5-ae41-8eb9b8048f2c) returns all Datapoints with region_id 6f5b625d-2eb4-44e5-ae41-8eb9b8048f2c

- [/readDatapoint/deviceid/e1bfc6cf-d387-4150-ba5c-9f4bb6694f14](https://us-central1-beacon-2de55.cloudfunctions.net/readDatapoint/regionid/e1bfc6cf-d387-4150-ba5c-9f4bb6694f14) returns the datapoint with device id e1bfc6cf-d387-4150-ba5c-9f4bb6694f14

- [/readDatapoint/id/16408793150665654](https://us-central1-beacon-2de55.cloudfunctions.net/readDatapoint/regionid/16408793150665654) returns the datapoint with unique database id 16408793150665654
