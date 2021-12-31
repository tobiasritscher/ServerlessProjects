# flutter_app

To run this flutter app the follow the steps in this tutorial: https://docs.flutter.dev/get-started/install
1. Install the Flutter SDK
2. Use the command flutter doctor to check if you installed the SDK correctly
3. Install the IDE: Android Studio (Windows/Linux) or Xcode (Mac)
4. Setup the Android Emulator (Windows/Linux) or the IOS simulator (Mac) and create a virtual device

Run the app
1. Open the project with the IDE
3. Run the ```flutter pub get``` command to install all dependencies
4. Use the command ```flutter run``` to run the app on the virtual device

## Functionality of the app
The app is a prove of concept since the real app would need text to speech and further improvements to be used by a blind person.
* You can enter your device_id and it will fetch all data points associated to it from the firebase database.
* The app updates the data points each 2 seconds to guarantee that the newest data points are loaded.
