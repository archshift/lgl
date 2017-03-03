#include <stddef.h>

struct LogBufferMutView {
    char* buf_ptr;
    size_t buf_size;
};

struct LogBufferView {
    const char* buf_ptr;
    size_t buf_size;
};

#ifdef __cplusplus
extern "C" {
#endif

LogBufferView lgl_buffer(LogBufferMutView buf);
size_t lgl_buffer_size();

#ifdef __cplusplus
}
#endif
