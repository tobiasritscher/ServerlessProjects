package com.example;

import com.google.api.core.ApiFuture;
import java.text.SimpleDateFormat;
import java.time.Instant;
import java.time.format.DateTimeFormatter;
import java.util.*;

import com.google.auth.oauth2.GoogleCredentials;
import com.google.cloud.firestore.Firestore;
import com.google.cloud.firestore.QueryDocumentSnapshot;
import com.google.cloud.firestore.QuerySnapshot;
import com.google.cloud.functions.HttpFunction;
import com.google.cloud.functions.HttpRequest;
import com.google.cloud.functions.HttpResponse;
import com.google.firebase.FirebaseApp;
import com.google.firebase.FirebaseOptions;
import com.google.firebase.cloud.FirestoreClient;
import com.google.gson.Gson;
import com.google.gson.JsonElement;
import com.google.gson.JsonObject;
import com.google.gson.JsonParseException;

import java.io.FileInputStream;
import java.io.IOException;
import java.io.PrintWriter;
import java.util.concurrent.ExecutionException;


public class Example implements HttpFunction {
    private static final Gson gson = new Gson();

    @Override
    public void service(HttpRequest request, HttpResponse response) throws IOException {
        String password = request.getFirstQueryParameter("password").orElse("false");
        var writer = new PrintWriter(response.getWriter());

        try {
            JsonElement requestParsed = gson.fromJson(request.getReader(), JsonElement.class);
            JsonObject requestJson = null;

            if (requestParsed != null && requestParsed.isJsonObject()) {
                requestJson = requestParsed.getAsJsonObject();
            }

            if (requestJson != null && requestJson.has("password")) {
                password = requestJson.get("name").getAsString();
            }
        } catch (JsonParseException ignored) {
        }

        if (password.equals("admin") || password.equals("clueless")) {
            HashMap<String, String> data = readDB();

            int feedbackCount = data.size();
            int WordCount = 0;
            int avarageWordcount = 0;

            for (String value : data.values()) {
                String[] words = value.split(" ");
                WordCount += words.length;
            }
            avarageWordcount = WordCount / feedbackCount;

            SortedSet<String> keys = new TreeSet<>(data.keySet());
            Date old = stringToDateTime(keys.first());
            Date date;
            int counter = 0;
            ArrayList<Integer> counterList = new ArrayList<>();

            for (String key: keys) {
                date = stringToDateTime(key);
                if ((date.getTime() - old.getTime())/1000/60/60 < 1) {
                    ++counter;
                } else {
                    counterList.add(counter);
                    counter = 0;
                }
            }
            counter = 0;
            for (int count: counterList) {
                counter += count;
            }

            writer.printf("Feedback counter: %s %nAvarage Word count: %s %nFeedbacks per hour: %f %nFirst Feedback: %s %nLast Feedback: %s", feedbackCount, avarageWordcount, (double)counter/counterList.size(), keys.first(), keys.last());
        } else {
            writer.printf("Missing or wrong password ('%s'). please enter the password at the end of the link like '/?password=xxx'", password);
        }
    }

    private Date stringToDateTime(String key) {
        return Date.from(Instant.from(DateTimeFormatter.ISO_OFFSET_DATE_TIME.parse(key)));
    }

    private HashMap<String, String> readDB() {
        Firestore db = null;
        try {
            db = setUpDBConnection();
        } catch (IOException e) {
            e.printStackTrace();
        }

        ApiFuture<QuerySnapshot> query = db.collection("feedbacks").get();
        QuerySnapshot querySnapshot = null;
        try {
            querySnapshot = query.get();
        } catch (InterruptedException | ExecutionException e) {
            e.printStackTrace();
        }
        List<QueryDocumentSnapshot> documents = querySnapshot.getDocuments();

        HashMap<String, String> data = new HashMap<>();
        for (QueryDocumentSnapshot document : documents) {
            data.put(document.getString("timestamp"), document.getString("feedback_text"));
        }

        return data;
    }

    boolean dbIsInitialized = false;
    private Firestore setUpDBConnection() throws IOException {
        if (!dbIsInitialized) {
            FileInputStream stream = new FileInputStream("key.json");

            FirebaseOptions options = new FirebaseOptions.Builder()
                    .setCredentials(GoogleCredentials.fromStream(stream))
                    .build();

            FirebaseApp.initializeApp(options);
            dbIsInitialized = true;
        }
        return FirestoreClient.getFirestore();
    }
}