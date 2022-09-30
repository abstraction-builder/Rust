#include <stdio.h>

int main()
{
	for (int i=0; i<20; i++)
	{
		fprintf(stdout, ".");
	}

	*((char*) 0xBADBADBAD) = 43;

	return 0;
}
