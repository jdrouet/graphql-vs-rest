import http from 'k6/http';
import { check } from 'k6';

export const options = {
    vus: 100,
    duration: '30s',
};

const URL = 'http://server-graphql-api:3000/';
const QUERY = '{ running }';

export default function () {
    const res = http.post(URL, JSON.stringify({ query: QUERY }));
    check(res, { 'status was 200': (r) => r.status == 200 });
}