Calculate Basic:
curl -X POST 'http://localhost:6767' -H 'Content-Type: application/json;charset=UTF-8'  --data-binary $'{ "a": true,"b": true, "c": true, "d": 10.0, "e": 5, "f": 8}'

Calculate v1:
curl -X POST 'http://localhost:6767/2' -H 'Content-Type: application/json;charset=UTF-8'  --data-binary $'{ "a": true,"b": false, "c": true, "d": 10.0, "e": 5, "f": 8}'

Calculate v2:
curl -X POST 'http://localhost:6767/2' -H 'Content-Type: application/json;charset=UTF-8'  --data-binary $'{ "a": true,"b": false, "c": true, "d": 10.0, "e": 5, "f": 8}'

