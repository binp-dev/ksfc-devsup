#include <stdio.h>
#include <stdint.h>
//#include <pthread.h>

//#include <string>
//#include <hash_map>


struct ev_loop *loop = ev_default_loop (0);

ev_io stdin_watcher;

ev_init (&stdin_watcher, my_cb);
ev_io_set (&stdin_watcher, STDIN_FILENO, EV_READ);
ev_io_start (loop, &stdin_watcher);

ev_run (loop, 0);

