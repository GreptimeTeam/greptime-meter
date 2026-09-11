.PHONY: header-check header-fix

header-check:
	docker run -it --rm -v "$(CURDIR):/github/workspace" apache/skywalking-eyes header check

header-fix:
	docker run -it --rm -v "$(CURDIR):/github/workspace" apache/skywalking-eyes header fix
