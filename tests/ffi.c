#include <stdio.h>
#include <stdint.h>

// todo-rs API:
typedef struct task_list_S task_list_t;
extern task_list_t * task_list_new(const char *name);
extern void task_list_free(task_list_t *);
extern void task_list_add(task_list_t *, const char *name);
extern void task_list_remove(task_list_t *, uint32_t num);
extern void task_list_toggle(task_list_t *, uint32_t num);

// DEBUG
extern void task_list_display(const task_list_t *);

int main(void) {
    task_list_t *list = task_list_new("Faire les courses");

    task_list_add(list, "Beurre");
    task_list_add(list, "Riz");
    task_list_add(list, "Cookies");

    task_list_toggle(list, 2);

    task_list_display(list);

    task_list_free(list);
}