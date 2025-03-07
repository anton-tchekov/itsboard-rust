all:
	nasm -f elf64 -o list.o list.S
	gcc -c main.c -o main.o
	gcc main.o list.o -o list
