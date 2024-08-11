typedef struct {
    uint8_t red;
    uint8_t green;
    uint8_t blue;
    uint8_t alpha;
} Color;

extern Color* update_and_render(size_t width, size_t height, float delta_time);
extern void free_buffer(Color* array, size_t length);
