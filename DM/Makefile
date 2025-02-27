CC = gcc
CFLAGS = -Wall -Wextra -O2
DEBUG_FLAGS = -Wall -Wextra -g -DDEBUG
LDFLAGS = -lm

all: tri_composite

tri_composite: tri_composite.c
	$(CC) $(CFLAGS) -o $@ $< $(LDFLAGS)

debug: tri_composite.c
	$(CC) $(DEBUG_FLAGS) -o tri_composite_debug $< $(LDFLAGS)

run: tri_composite
	./tri_composite

clean:
	rm -f tri_composite tri_composite_debug

.PHONY: all debug run clean