import 'package:flutter/material.dart';
import 'package:flutter_app/data_point_controller.dart';

import 'data_points_model.dart';
import 'package:intl/intl.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      debugShowCheckedModeBanner: false,
      title: 'Flutter Demo',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: const MyHomePage(),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({Key? key}) : super(key: key);
  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  DataPointsController dataPointsController = DataPointsController();
  final _formKey = GlobalKey<FormState>();
  final TextEditingController deviceIdController =
      TextEditingController(text: "e1bfc6cf-d387-4150-ba5c-9f4bb6694f14");

  @override
  void initState() {
    periodicUpdater();
    super.initState();
  }

  void periodicUpdater() async {
    while (mounted) {
      await Future.delayed(const Duration(seconds: 2));
      setState(() {
        //dataPointsController.getAllDataPoints();
        print("UPDATE");
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("ProLoc"),
      ),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            Form(
                key: _formKey,
                child: Column(
                  children: [
                    const Text(
                      "Please enter your device id",
                      style:
                          TextStyle(fontSize: 20, fontWeight: FontWeight.bold),
                    ),
                    TextFormField(
                      textAlign: TextAlign.center,
                      controller: deviceIdController,
                    ),
                    ElevatedButton(
                      onPressed: () {
                        if (_formKey.currentState!.validate()) {
                          setState(() {
                            dataPointsController.deviceId =
                                deviceIdController.text;
                            dataPointsController.getAllDataPoints();
                          });
                        }
                      },
                      child: const Text('Submit'),
                    ),
                  ],
                )),
            Expanded(
              child: FutureBuilder<List<DataPoint>>(
                builder: (context, dataPointSnap) {
                  if (dataPointSnap.connectionState == ConnectionState.none &&
                      dataPointSnap.hasData) {
                    //print('project snapshot data is: ${projectSnap.data}');
                    return const CircularProgressIndicator();
                  }
                  List<DataPoint>? dataPointList =
                      dataPointSnap.data?.reversed.toList() ?? [];
                  return ListView.builder(
                    itemCount: dataPointList.length,
                    itemBuilder: (context, index) {
                      DataPoint dataPoint = dataPointList[index];
                      DateTime dateTime = DateTime.parse(dataPoint.timestamp);
                      String formattedDate =
                          DateFormat('yyyy-MM-dd – kk:mm').format(dateTime);
                      return Card(
                        margin: const EdgeInsets.all(10),
                        child: Container(
                          padding: const EdgeInsets.fromLTRB(10, 5, 10, 5),
                          child: Column(
                            children: [
                              Text(formattedDate),
                              Text(
                                dataPoint.data,
                                style: const TextStyle(fontSize: 20),
                              ),
                            ],
                          ),
                        ),
                      );
                      // ignore: dead_code
                      return Column(
                        children: <Widget>[Text(dataPoint.data)],
                      );
                    },
                  );
                },
                future: dataPointsController.getAllDataPoints(),
              ),
            )
          ],
        ),
      ),
      /*floatingActionButton: FloatingActionButton(
          onPressed: _incrementCounter,
          tooltip: 'Increment',
          child: const Icon(Icons.add),
        )*/
    );
  }
}
