# rust-webserver
a simple and fast webserver implimented in rust

# http request and response headers

GET /index.html HTTP/1.0\r\n
Accept: text/*\r\n
Accept-Language: en, fr

HTTP/1.0 200 OK\r\n
Content-type: text/plain\r\n
Content-length: 19\r\n\r\nhi! I'm a message\r\n
