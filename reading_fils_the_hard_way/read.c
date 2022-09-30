#include <stdlib.h>
#include <stdio.h>
#include <stdbool.h>

int main(int argc, char* argv[])
{
	FILE *file = fopen("/etc/hosts", "r");
	if (!file)
	{
		fprintf(stderr, "could not open file\n");
		exit(1);
	}

	const size_t buffer_size = 8;
	char *buf = malloc(buffer_size);

	while (true)
	{
		size_t size = fread(buf, 1, buffer_size - 1, file);
		if (size == 0)
		{
			break;
		}

		buf[size] = '\0';
		printf("%s", buf);
	}

	printf("\n");
	return 0;
}
