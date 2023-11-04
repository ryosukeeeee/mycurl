# mycurl

これは2023年11月3日と4日の2日間かけて開催されたRust勉強会の演習課題の模範解答です。

## 実行例

このサンプルの実行前に別ターミナルから`cargo run -p print-server`してローカルホストの3000番にサーバを起動しています。

```
$ curl http://localhost:3000
Hello from `GET /`
$ curl -X POST http://localhost:3000
Hello from `POST /`
$ curl -X GET http://localhost:3000 
Hello from `GET /`
$
$ ./target/debug/mycurl http://localhost:3000
Hello from `GET /`
$ ./target/debug/mycurl -X POST http://localhost:3000
Hello from `POST /`
$ ./target/debug/mycurl -X GET http://localhost:3000 
Hello from `GET /`
$ ./target/debug/mycurl -h                          
Usage: mycurl [OPTIONS] <URL>

Arguments:
  <URL>  ex. http://example.com

Options:
  -X, --request <method>  
  -h, --help              Print help
```
