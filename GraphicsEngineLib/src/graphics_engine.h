typedef struct {
    uint8_t red;
    uint8_t green;
    uint8_t blue;
    uint8_t alpha;
} Color;

extern void create_scene(void);
extern Color* update_and_render(int32_t width, int32_t height, float delta_time);
extern void free_bitmap(Color* array, size_t length);
