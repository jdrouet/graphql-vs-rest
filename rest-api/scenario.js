import http from 'k6/http';

import { check } from 'k6';

export const options = {
    vus: 100,
    duration: '30s',
};

const URL = 'http://server-rest-api:3000/status';

export default function () {
    const res = http.get(URL);
    check(res, { 'status was 200': (r) => r.status == 200 });
}