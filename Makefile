
all: test_openat

test_openat: test_openat.c
	$(CC) $(CFLAGS) -o openat test_openat.c

clean veryclean:
	$(RM) Hello