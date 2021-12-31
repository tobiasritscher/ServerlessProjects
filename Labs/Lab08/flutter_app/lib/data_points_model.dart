import 'package:flutter/foundation.dart';

class DataPoint {
  final String timestamp;
  final String id;
  final String deviceData;
  final String regionId;
  final String data;

  DataPoint(
      {required this.timestamp,
      required this.id,
      required this.deviceData,
      required this.regionId,
      required this.data});

  factory DataPoint.fromJson(Map<String, dynamic> json) {
    return DataPoint(
        timestamp: json['timestamp'] as String,
        id: json['id'] as String,
        deviceData: json['device_data'] as String,
        regionId: json["region_id"] as String,
        data: json["data"] as String);
  }
}
