#include <stdio.h>
#include <stdlib.h>
#include <fcntl.h>
#include <unistd.h>

int main(int argc, char* argv[])
{
	int fd = open("/etc/hosts", O_RDONLY);

	printf("Our file descriptor for /etc/hosts is %d\n", fd);
	printf("Press enter to exit...\n");
	getc(stdin);
}
