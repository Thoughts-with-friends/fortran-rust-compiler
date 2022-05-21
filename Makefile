default: test

build:
		cargo build --verbose
test: build
		cargo test --verbose
run:
		cargo run


# ----- git
ifeq ($(OS),Windows_NT)
		SHELL := pwsh.exe
else
		SHELL := bash
endif
current_branch := $(shell $(SHELL) -c 'git rev-parse --abbrev-ref HEAD')

g-mk:
		python3 "./scripts/create-branch.py"

g-pull:
		git stash --include-untracked
		git checkout main
		git pull
		git fetch -p
		git checkout $(current_branch)
		git stash pop

g-pr:
		gh pr create -w $(current_branch)
