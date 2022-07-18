#include "foo.h"
#include <stdlib.h>

void foo() {
    void* buf = malloc(4);
    (void)buf;
}

