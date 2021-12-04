CC = clang
CFLAGS = `cat ./compile_flags.txt`
SRC_DIR = src
BUILD_DIR = build

run: compile
	$(BUILD_DIR)/aoc

compile: clean
	mkdir build/
	$(CC) $(CFLAGS) -o build/aoc $(SRC_DIR)/aoc.c

clean:
	rm -rf $(BUILD_DIR)
