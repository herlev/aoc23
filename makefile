.PHONY = run test init testo wrun push

day != date +%d

run:
	cargo run --bin $(day)

test:
	cargo test --bin $(day)

wrun:
	cargo watch -qcs "make --silent run"

# Run tests showing output
testo:
	cargo test --bin $(day) -- --nocapture

# Copy template file
src/bin/%.rs: src/bin
	./gen-day.sh $*

init: src/bin/$(day).rs
	mkdir -p {puzzles,inputs,src/bin}
	aoc download -d $(day) -i inputs/$(day).txt -p puzzles/$(day).md --overwrite
	env PAGER="less -r" glow -p puzzles/$(day).md
	
push:
	git add src/bin/$(day).rs
	git commit -m "add day $(day)"
	git push
