Simple http server and  http(s) client

Server will get image location via http parameters and return URI for resized image.

Suddenly, SSL support requires rustls which doesn't compile on gnu toolchain.
REQUIRED
rustup default stable-x86_64-pc-windows-msvc 

/static/ folder should exist (at server executable working directory)

NOTE
Not all URI will be available from browser because browser doesn't encode URI path.

USAGE
client.exe "URI"
or with browser
http://127.0.0.1:8080/load_file?image_url=URI
