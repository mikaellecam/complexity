CC = gcc
CFLAGS = -Wall -Wextra -O2
DEBUG_FLAGS = -Wall -Wextra -g -DDEBUG
LDFLAGS = -lm

PROG = deux_elements

SRC = $(PROG).c

all: $(PROG)

$(PROG): $(SRC)
	$(CC) $(CFLAGS) -o $@ $< $(LDFLAGS)

debug: CFLAGS = $(DEBUG_FLAGS)
debug: $(PROG)_debug

$(PROG)_debug: $(SRC)
	$(CC) $(DEBUG_FLAGS) -o $@ $< $(LDFLAGS)

run: $(PROG)
	./$(PROG)

clean:
	rm -f $(PROG) $(PROG)_debug

.PHONY: all debug run clean