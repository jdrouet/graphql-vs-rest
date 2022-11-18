import http from 'k6/http';
import { check } from 'k6';
import { randomString } from 'https://jslib.k6.io/k6-utils/1.2.0/index.js';

export const options = {
  vus: 10,
  duration: '30s',
};

const HOST = 'http://server-rest-api:3000';
const headers = { 'Content-Type': 'application/json' };

const createName = () => randomString(8, 'aeioubcdfghijpqrstuv') + ' ' + randomString(8, 'aeioubcdfghijpqrstuv');
const createEmail = () => randomString(8, 'aeioubcdfghijpqrstuv') + '@' + randomString(8, 'aeioubcdfghijpqrstuv');

export default function () {
  const status = http.get(`${HOST}/status`);
  check(status, { 'status was 200': (r) => r.status == 200 });
  const create_account = http.post(`${HOST}/accounts`, JSON.stringify({
    name: createName(),
    email: createEmail(),
  }), { headers });
  check(create_account, { 'create account was 200': (r) => r.status == 200 });
}