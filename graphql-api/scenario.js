import http from 'k6/http';
import { check, group } from 'k6';
import { randomString } from 'https://jslib.k6.io/k6-utils/1.2.0/index.js';

export const options = {
  vus: 10,
  duration: '30s',
};

const URL = 'http://server-graphql-api:3000/';

const createName = () => randomString(8, 'aeioubcdfghijpqrstuv') + ' ' + randomString(8, 'aeioubcdfghijpqrstuv');
const createEmail = () => randomString(8, 'aeioubcdfghijpqrstuv') + '@' + randomString(8, 'aeioubcdfghijpqrstuv');
const createMessage = () => randomString(30);

export default function () {
  group('status', function () {
    const res = http.post(URL, JSON.stringify({ query: '{ running }' }));
    check(res, {
      'status was 200': (r) => r.status === 200,
      'payload': (r) => r.json().data.running === true,
    });
  });
  //
  const accountId = group('create account', function () {
    const query = `
  mutation CreateAccount {
    createAccount(input:{name:"${createName()}",email:"${createEmail()}"}) {
      id
      name
      email
      createdAt
    }
  }`;
    const req = http.post(URL, JSON.stringify({ query }));
    check(req, {
      'status was 200': (r) => r.status === 200,
      'payload': (r) => !!r.json().data.createAccount.id,
    });
    return req.json().data.createAccount.id;
  });
  //
  group('create message', function () {
    const query = `
  mutation CreateMessage {
    createMessage(input:{creatorId:"${accountId}",content:"${createMessage()}"}) {
      id
      content
      createdBy
      createdAt
    }
  }`;
    const req = http.post(URL, JSON.stringify({ query: query }));
    check(req, {
      'status was 200': (r) => r.status === 200,
      'payload': (r) => {
        const payload = r.json();
        return !!payload.data && !!payload.data.createMessage && !!payload.data.createMessage.id;
      },
    });
  });
  //
  group('list messages with user', function () {
    const query = `
  query ListMessages {
    messages(input:{count:50,page:0}) {
      id
      content
      creator {
        id
        name
        email
        createdAt
      }
      createdAt
    }
  }`;
    const req = http.post(URL, JSON.stringify({ query: query }));
    check(req, {
      'status was 200': (r) => r.status === 200,
      'payload': (r) => {
        const payload = r.json();
        return !!payload.data && !!payload.data.messages && Array.isArray(payload.data.messages);
      },
    });
  });
}