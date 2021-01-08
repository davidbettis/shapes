default: build

build:
	wasm-pack build --release --target web
	mkdir -p build
	cp -R index.js index.html pkg build/

# curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
server:
	http

deploy: clean build
	aws s3 sync ./build ${SHAPES_S3}

clean:
	rm -rf pkg build
