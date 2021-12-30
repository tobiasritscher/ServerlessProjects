const functions = require("firebase-functions");
const admin = require('firebase-admin');

admin.initializeApp();  
const db = admin.firestore();

// URL: https://us-central1-beacon-2de55.cloudfunctions.net

exports.createDatapoint = functions.https.onRequest((req, res) => {
    (async () => {
        try {
          const n = Math.floor(Math.random()*10000);
          const time_id = parseInt(Date.now()+""+n);
          await db.collection('datapoint').doc('/' + time_id + '/')
              .create({
              id: req.body.id,
              data: req.body.data,
              //region_id: (typeof req.body.region_id === 'undefined') ? 0 : req.body.region_id,
              //device_data: (typeof req.body.device.data === undefined) ? 0 : req.body.device.data,
              region_id: req.body.region_id,
              device_data: req.body.device_data,
              timestamp: req.body.timestamp
            });
          res.status(200).send();
        } catch (error) {
          res.status(500).send(error);
        }
      })();
});

exports.readAllDatapoints = functions.https.onRequest((req, res) => {
    (async () => {
        try {
            let query = db.collection('datapoint');
            let response = [];
            await query.get().then(querySnapshot => {
            let docs = querySnapshot.docs;
            for (let doc of docs) {
                response.push(doc.data());
            }
            });
            res.status(200).send(response);
        } catch (error) {
            res.status(500).send(error);
        }
    })();
});

const express = require('express');
const cors = require('cors');
const app = express();
app.use(cors({ origin: true }));

app.get('/id/:id', (req, res) => {
    (async () => {
        try {
            const document = db.collection('datapoint').doc(req.params.id);
            let datapoint = await document.get();
            let response = datapoint.data();
            res.status(200).send(response);
        } catch (error) {
            res.status(500).send(error);
        }
    })();
});

app.get('/regionid/:regionid', (req, res) => {
    (async () => {
        try {
            const query = db.collection('datapoint');

            let response = []
            const region_id = req.params.regionid

            var querySnapshot = await query.where('region_id','==',region_id).get();   
            querySnapshot.forEach(doc => {
                response.push(doc.data());
            });
            res.status(200).send(response);
        } catch (error) {
            res.status(500).send(error);
        }
    })();
});

app.get('/deviceid/:deviceid', (req, res) => {
    (async () => {
        try {
            const query = db.collection('datapoint');

            let response = []
            const device_id = req.params.deviceid

            var querySnapshot = await query.where('id','==',device_id).get();   
            querySnapshot.forEach(doc => {
                response.push(doc.data());
            });
            res.status(200).send(response);
        } catch (error) {
            res.status(500).send(error);
        }
    })();
});

exports.readDatapoint = functions.https.onRequest(app);
