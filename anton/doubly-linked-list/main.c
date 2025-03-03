#include <stdio.h>
#include <stdint.h>

typedef struct NODE
{
	struct NODE *Prev;
	struct NODE *Next;
	void *Value;
} Node;

typedef struct
{
	Node *Head;
	Node *Tail;
	uint64_t Count;
} List;

void *peek_min(List *list);
void *peek_max(List *list);
void *pop_min(List *list);
void *pop_max(List *list);
void insert(List *list, Node *val);
void iter(List *list, void (*fn)(Node *node, void *context), void *context);

void fn(Node *node, void *ctx)
{
	printf("FN: %p\n", node->Value);
}

int main(void)
{
	printf("List Test\n");

	List list = { NULL, NULL, 0 };

	Node v = { NULL, NULL, (void *)5 };

	insert(&list, &v);

	printf("%p %p %lld\n", list.Head, list.Tail, list.Count);

	iter(&list, fn, NULL);

	void *n = peek_min(&list);
	printf("%p\n", n);

	void *b = pop_min(&list);
	printf("%p\n", b);

	printf("%p %p %lld\n", list.Head, list.Tail, list.Count);


	return 0;
}
