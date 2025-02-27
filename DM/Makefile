CC = gcc
CFLAGS = -Wall -Wextra -O2
DEBUG_FLAGS = -Wall -Wextra -g -DDEBUG
LDFLAGS = -lm

all: tri_composite deux_elements

tri_composite: tri_composite.c
	$(CC) $(CFLAGS) -o $@ $< $(LDFLAGS)

deux_elements: deux_elements.c
	$(CC) $(CFLAGS) -o $@ $< $(LDFLAGS)

debug_tri: tri_composite.c
	$(CC) $(DEBUG_FLAGS) -o tri_composite_debug $< $(LDFLAGS)

debug_deux: deux_elements.c
	$(CC) $(DEBUG_FLAGS) -o deux_elements_debug $< $(LDFLAGS)

run_tri: tri_composite
	./tri_composite

run_deux: deux_elements
	./deux_elements

# Clean up
clean:
	rm -f tri_composite tri_composite_debug deux_elements deux_elements_debug

.PHONY: all debug_tri debug_deux run_tri run_deux clean