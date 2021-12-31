// This is a basic Flutter widget test.
//
// To perform an interaction with a widget in your test, use the WidgetTester
// utility that Flutter provides. For example, you can send tap and scroll
// gestures. You can also use WidgetTester to find child widgets in the widget
// tree, read text, and verify that the values of widget properties are correct.

import 'package:flutter/material.dart';
import 'package:flutter_app/data_point_controller.dart';
import 'package:flutter_test/flutter_test.dart';

import 'package:flutter_app/main.dart';

void main() {
  final DataPointsController dataPointsController = new DataPointsController();

  testWidgets('GetAllDataPoints test', (WidgetTester tester) async {
    await dataPointsController.getAllDataPoints();
    print("FirstElem" + dataPointsController.dataPointList[0].data);
    expect([], dataPointsController.dataPointList);
  });
}
