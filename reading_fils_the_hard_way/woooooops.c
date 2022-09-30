#include <unistd.h>

int main()
{
	for (int i=0; i<20; i++)
	{
		write(STDOUT_FILENO, ".", 1);
	}

	*((char*) 0xBADBADBAD) = 15213;

	return 0;
}
