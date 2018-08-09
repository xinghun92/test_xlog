test:
	scp /Users/wangsijie/rust/mars/libxlog.a  ./
	cargo test

bind:
	bindgen xloggerbase.h -o bindgen.rs
	mv bindgen.rs src/xlog.rs