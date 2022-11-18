import http from 'k6/http';
import { check, group } from 'k6';
import { randomString } from 'https://jslib.k6.io/k6-utils/1.2.0/index.js';

export const options = {
  vus: 10,
  duration: '30s',
};

const HOST = 'http://server-rest-api:3000';
const headers = { 'Content-Type': 'application/json' };

const createName = () => randomString(8, 'aeioubcdfghijpqrstuv') + ' ' + randomString(8, 'aeioubcdfghijpqrstuv');
const createEmail = () => randomString(8, 'aeioubcdfghijpqrstuv') + '@' + randomString(8, 'aeioubcdfghijpqrstuv');
const createMessage = () => randomString(30);

export default function () {
  group('status', function () {
    const req = http.get(`${HOST}/status`);
    check(req, { 'status was 200': (r) => r.status == 200 });
  });
  //
  const accountId = group('create account', function () {
    const req = http.post(`${HOST}/accounts`, JSON.stringify({
      name: createName(),
      email: createEmail(),
    }), { headers });
    check(req, { 'create account was 200': (r) => r.status == 200 });
    return req.json().id;
  });
  //
  group('create message', function () {
    const createdMessage = http.post(`${HOST}/messages`, JSON.stringify({
      content: createMessage(),
      creatorId: accountId,
    }), { headers });
    check(createdMessage, { 'create message was 200': (r) => r.status == 200 });
  });
}