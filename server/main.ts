import cors from 'cors';
import express from 'express';

require('../gulpfile');

const app = express();
const port = process.env.PORT || 4000;

const isProduction = 'production' === process.env.NODE_ENV;

app.use(cors());

if (isProduction) {
  app.use(express.static('public'));
}

app.listen(port, () => {});
