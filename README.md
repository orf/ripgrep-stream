# ripgrep-stream

Proof of concept to illustrate stream matching with user-supplied file names

```shell
$ cat example_json.txt | cargo run -- '^he.*$\nworld$'
id_one:hell! no!
id_one:world
id_two:hello
id_two:world
```
