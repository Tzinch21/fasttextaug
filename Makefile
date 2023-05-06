reinstall:
	pip uninstall -y -q -q fasttextaug
	rm -f rust/target/wheels/*
	maturin build
	$(eval WHEEL := $(shell ls rust/target/wheels))
	pip install rust/target/wheels/$(WHEEL)
test:
	cargo test --manifest-path rust/Cargo.toml
