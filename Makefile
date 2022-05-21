default: test

build:
		cargo build --verbose
test: build
		cargo test --verbose
run:
		cargo run


# ----- git
ifeq ($(OS),Windows_NT)
		SHELL := powershell.exe
else
		SHELL := bash
endif
current_branch := $(shell $(SHELL) -c 'git rev-parse --abbrev-ref HEAD')

# create branch
br:
		python3 "./scripts/create-branch.py"

# udate main repository
update:
		git stash --include-untracked
		git checkout main
		git pull
		git fetch -p
		git checkout $(current_branch)
		git stash pop

# crete pull request
git-pr:
		gh pr create -w $(current_branch)
