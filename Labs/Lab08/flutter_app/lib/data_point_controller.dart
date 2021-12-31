import 'dart:convert';
import 'package:flutter_app/data_points_model.dart';
import 'package:http/http.dart';

class DataPointsController {
  final _uri = "https://us-central1-beacon-2de55.cloudfunctions.net";
  List<DataPoint> dataPointList = [];
  String deviceId = "e1bfc6cf-d387-4150-ba5c-9f4bb6694f14";

  List<DataPoint> _dataPointsFromJson(postJSON) {
    List<dynamic> body = jsonDecode(postJSON) as List<dynamic>;
    List<DataPoint> dataPoints =
        body.map((dynamic item) => DataPoint.fromJson(item)).toList();
    return dataPoints;
  }

  Future<List<DataPoint>> getAllDataPoints() async {
    Response response = await get(
      Uri.parse(_uri + "/readDatapoint/deviceid/" + deviceId),
      //headers: <String, String>{ 'Content-Type': 'application/json; charset=UTF-8',}
    );
    if (response.statusCode == 200) {
      dataPointList = _dataPointsFromJson(response.body);
    } else {
      dataPointList = [];
    }
    print(response.body);
    return dataPointList;
  }
}
