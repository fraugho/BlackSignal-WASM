# BlackSignal-WASM

# How To Run
```
redis-server --port 6379 --bind 127.0.0.1 --save "" --appendonly no

surreal start --log trace --user root --pass root --bind 127.0.0.1:8000 memory

cd Backend
cargo run --release

cd ..
cd Frontend
trunk serve --port 3000
```
