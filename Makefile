run:
	cargo xrun

test:
	cargo xtest

git:
	git diff
	git add .
	git commit
	git push

.PHONY: run test git

