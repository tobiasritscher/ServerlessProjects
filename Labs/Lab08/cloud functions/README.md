URL: https://us-central1-beacon-2de55.cloudfunctions.net

Endpoints:


POST request:

- /createDatapoint

creates a new datapoint

{

    "id":165623652695468451651,

    "data":"Some more data for another datapoint",

    "region_id": 597956666487987,

    "device_data":"test devide data",

    "timestamp":"2021-12-28T12:30:39.615678876Z"

}



GET requests:

- /readAllDatapoints

returns all datapoints

- /readDatapoint/regionid/597956666487987

returns all Datapoints with region_id 597956666487987

- /readDatapoint/id/159763184614576

returns the datapoint with id 159763184614576
