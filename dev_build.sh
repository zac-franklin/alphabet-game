sudo docker build -f docker/Dockerfile.dev -t alphabet-game:dev docker
sudo docker run -it --mount type=bind,source=$(pwd)/code,target=/build alphabet-game:dev /bin/bash -c "cargo build --release --target wasm32-unknown-unknown && wasm-bindgen --out-name alphabet-game --out-dir pkg --target web target/wasm32-unknown-unknown/release/alphabet_game.wasm"
sudo docker build -f docker/Dockerfile.serve -t alphabet-game:serve code/
sudo docker run -p 80:80 alphabet-game:serve