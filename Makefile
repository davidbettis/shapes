default:
	wasm-pack build --release --target web

# curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
server:
	http

clean:
	rm -rf pkg
