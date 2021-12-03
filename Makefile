CC := gcc
CFLAGS := -fPIC -g -Wall -O2 -Wc++-compat #-Wextra
.c.o:
	$(CC) -c $(CFLAGS) $< -o $@

main: ./src/test.o ./src/main.o
	$(CC) $(CFLAGS) -c ./src/test.c -o
