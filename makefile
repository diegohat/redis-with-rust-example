setup-hooks:
	@echo "Installing required tools..."
	@cargo install cargo-audit --features=fix
	@echo "Installing git hooks..."
	@lefthook install
	@echo "Setup completed successfully!"