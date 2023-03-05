
setup:
	mkdir ~/skate/
	mkdir ~/skate/bin
	mkdir ~/skate/deps
	mkdir ~/skate/global-packages
compile:
	cargo build --release
	mv ./target/release/skate ~/skate/bin

	echo "Please add skate path to your .bashrc file"
