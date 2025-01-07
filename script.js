import http from 'k6/http';

export const options = {
  // A number specifying the number of VUs to run concurrently.
  vus: 10,
  // A string specifying the total duration of the test run.
  duration: '10s',

};

export default function() {
  http.get('http://localhost:8080');
}