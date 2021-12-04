# rust-ffi-cpp-simple
最小構成

## build

```
gcc -fPIC -g -Wall -O2 -Wc++-compat -c ./src/test.c -o ./target/libtest.a
```


```
cargo run
```

## ref

https://bitbucket.org/ban-m/call_c_function_from_rust/src