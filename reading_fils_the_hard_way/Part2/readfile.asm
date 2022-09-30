global _start
section .text

_start:
	mov		rax, 2		; "open" syscall
	mov		rdi, path	; arg 1: path
	xor		rsi, rdi	; arg 2: flags (0 = O_RDONLY)
	syscall

	mov 	rax, 60		; "exit" syscall
	syscall

section .data

path:
	dn		"/etc/hosts", 0 ; null-terminated
