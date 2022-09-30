		global _start

		section .text
_start:
		mov		rax, 2		; "open" syscall
		mov		rdi, path	; arg 1: path
		xor		rsi, rsi	; arg 2: flags (0 = O_RDONLY)
		syscall

		mov 	rax, 60		; "exit" syscall
		xor 	rdi, rdi	; <---- exit with code 0
		syscall

		section .data
path:	db		"/etc/hosts", 0 ; null-terminated
