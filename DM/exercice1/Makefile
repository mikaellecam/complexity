CC = gcc
CFLAGS = -Wall -Wextra -O2
DEBUG_FLAGS = -Wall -Wextra -g -DDEBUG
LDFLAGS = -lm

TRI_SRC = tri_composite.c sorting_algorithms.c utility.c
TRI_OBJ = $(TRI_SRC:.c=.o)

all: tri_composite

tri_composite: $(TRI_OBJ)
	$(CC) $(CFLAGS) -o $@ $^ $(LDFLAGS)

%.o: %.c tri_composite.h
	$(CC) $(CFLAGS) -c $< -o $@

debug: CFLAGS = $(DEBUG_FLAGS)
debug: tri_composite

run: tri_composite
	./tri_composite

# Clean up
	rm -f tri_composite $(TRI_OBJ)

.PHONY: all debug run clean