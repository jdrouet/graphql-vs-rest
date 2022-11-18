import http from 'k6/http';
import { check } from 'k6';
import { randomString } from 'https://jslib.k6.io/k6-utils/1.2.0/index.js';

export const options = {
  vus: 10,
  duration: '30s',
};

const URL = 'http://server-graphql-api:3000/';

const createName = () => randomString(8, 'aeioubcdfghijpqrstuv') + ' ' + randomString(8, 'aeioubcdfghijpqrstuv');
const createEmail = () => randomString(8, 'aeioubcdfghijpqrstuv') + '@' + randomString(8, 'aeioubcdfghijpqrstuv');

export default function () {
  const status = http.post(URL, JSON.stringify({ query: '{ running }' }));
  check(status, { 'status was 200': (r) => r.status == 200 });
  const query = `
mutation CreateAccount {
  createAccount(input:{name:"${createName()}",email:"${createEmail()}"}) {
    id
    name
    email
    createdAt
  }
}`;
  const create = http.post(URL, JSON.stringify({ query }));
  check(create, { 'create account was 200': (r) => r.status == 200 });
}