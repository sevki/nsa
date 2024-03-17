#include <fcntl.h>
#include <stdio.h>
#include <unistd.h>

int main() {
    int fd = openat(AT_FDCWD, "Cargo.toml", O_RDONLY);
    if (fd == -1) {
        perror("openat");
        return 1;
    }
    close(fd);
    return 0;
}