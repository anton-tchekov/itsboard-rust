global peek_max
global peek_min
global pop_max
global pop_min
global insert
global iter

; List
OFFSET_HEAD  equ  0
OFFSET_TAIL  equ  8
OFFSET_COUNT equ 16

; Node
OFFSET_PREV  equ  0
OFFSET_NEXT  equ  8
OFFSET_VALUE equ 16

SECTION .text

; >>> Returns minimum element without removing it
; Params:  RDI: List
; Returns: RAX: Value
peek_min:
	; Get head
	mov rsi, [rdi + OFFSET_HEAD]
	; Check for null
	cmp rax, 0
	jnz .not_null
	ret
.not_null:
	; Get element value
	mov rax, [rsi + OFFSET_VALUE]
	ret

; >>> Returns minimum element and removes it
; Params:  RDI: List
; Returns: RAX: Value
pop_min:
	; Get head
	mov rsi, [rdi + OFFSET_HEAD]
	; Check for null
	cmp rax, 0
	jnz .not_null
	ret
.not_null:
	; Decrement count
	dec dword [rdi + OFFSET_COUNT]
	; If node->next == NULL
	mov r8, [rsi + OFFSET_NEXT]
	; Fix head
	mov [rdi + OFFSET_HEAD], r8
	cmp r8, 0
	jnz .multi
	; Fix tail
	mov [rdi + OFFSET_TAIL], r8
	jmp .end
.multi:
	; Else fix pointers of next
	mov r9, [r8]
	xor rax, rax
	mov [r9 + OFFSET_PREV], rax
.end:
	; Get element value
	mov rax, [rsi + OFFSET_VALUE]
	ret

; List
; Returns minimum element without removing it
peek_max:
	ret

; List
; Returns maximum element
pop_max:
	ret

; Calls a function pointer for each element
; Params: RDI: List, RSI: Function pointer, RDX: Context param
iter:
	push r12
	push r13
	push r14
	push r15

	mov r14, rdx
	mov r13, [rdi + OFFSET_HEAD]
	mov r12, rsi
.loop
	cmp r13, 0
	jz .end
	mov rdi, r13
	mov rsi, r14
	call r12
	mov r13, [r13 + OFFSET_NEXT]
	jmp .loop
.end:

	pop r15
	pop r14
	pop r13
	pop r12
	ret

; >>> Insert value in list
; Params: RDI: List, RSI: Node, RDX: Comparator Fn
insert:
	; If empty
	mov r8, [rdi + OFFSET_COUNT]
	cmp r8, 0
	jnz .not_empy

	xor rax,  rax
	mov [rsi + OFFSET_PREV], rax
	mov [rsi + OFFSET_NEXT], rax

	mov [rdi + OFFSET_HEAD], rsi
	mov [rdi + OFFSET_TAIL], rsi
	jmp .end
.not_empy:
	; List already contains elements

	mov r12, [rdi + OFFSET_HEAD]
.loop:


	jmp .loop

.end:

	; Increment count
	inc dword [rdi + OFFSET_COUNT]

	ret
