.PHONY: help install dev start check typecheck lint fix format build graph release

help:
	@echo "install dev start check typecheck lint fix format build graph release"

install:
	npm ci

dev start check typecheck lint fix format build graph:
	npm run $@

release:
	npm run build:release
