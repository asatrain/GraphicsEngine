//
//  AppDelegate.m
//  GraphicsEngineMacOS
//
//  Created by Emin Asatrian on 19.07.2024.
//

#import "AppDelegate.h"

@interface AppDelegate ()

@property (strong) NSWindow* window;
@property uint64_t prevTimeNs;

@end

@implementation AppDelegate

- (void)engineUpdate {
    uint64_t currentTimeNs = mach_absolute_time();
    uint64_t deltaTimeNs = currentTimeNs - self.prevTimeNs;
    NSLog(@"%f ms between", deltaTimeNs / 1000.0f);
//    NSLog(@"%f fps", 1000000.0f / (currentTimeNs - self.prevTimeNs));
//    [self.window setTitle:
//         [NSString stringWithFormat: @"Best Game in the world %d ms",
//          (int)(1000000.0f / (currentTimeNs - self.prevTimeNs))]
//    ];
    self.prevTimeNs = currentTimeNs;
    
    size_t width = self.window.contentView.bounds.size.width;
    size_t height = self.window.contentView.bounds.size.height;
    
    CGColorSpaceRef rgb = CGColorSpaceCreateWithName(kCGColorSpaceSRGB);
    CGContextRef bitmapCtx = CGBitmapContextCreate(NULL, width, height, 8, 0, rgb, kCGImageByteOrder32Big | kCGImageAlphaPremultipliedLast);
    CGColorSpaceRelease(rgb);
    
    size_t pitch = CGBitmapContextGetBytesPerRow(bitmapCtx);
    uint8_t *buffer = CGBitmapContextGetData(bitmapCtx);
    
    Pixel color = { .red=200, .green=0, .blue=0, .alpha=255 };
    
    for (size_t y = 0; y < height; ++y) {
        Pixel *row = (Pixel *)(buffer + y * pitch);
        for (size_t x = 0; x < width; ++x) {
            row[x] = color;
        }
    }
    
    Pixel green = { .red=0, .green=200, .blue=0, .alpha=255 };
    for (size_t i = 17000; i < 19000; ++i)
        ((Pixel*)buffer)[i] = green;
    
    CGImageRef image = CGBitmapContextCreateImage(bitmapCtx);
//    self.window.contentView.wantsLayer = YES;
    self.window.contentView.layer.contents = (__bridge id)image;
    CGContextRelease(bitmapCtx);
    CGImageRelease(image);
    
    [self.window.contentView display];
}

- (void)applicationDidFinishLaunching:(NSNotification *)aNotification {
    // Insert code here to initialize your application
    
    NSMenu* mainMenu = [NSApplication sharedApplication].mainMenu;
    NSMenuItem* appMenuItem = [[NSMenuItem alloc] initWithTitle:@"Quit" action:@selector(terminate:) keyEquivalent:@"q"];
    [mainMenu.itemArray[0].submenu addItem:appMenuItem];
    
    self.window = [[NSWindow alloc] initWithContentRect:NSMakeRect(0, 0, 800, 450) styleMask:(NSWindowStyleMaskMiniaturizable | NSWindowStyleMaskClosable | NSWindowStyleMaskResizable | NSWindowStyleMaskTitled)
                                                backing:NSBackingStoreBuffered
                                                  defer:NO];
    [self.window setMinSize:NSMakeSize(320, 180)];
    [self.window center];
    [self.window setTitle: @"Best Game in the world"];
    [self.window makeKeyAndOrderFront:self];
    
    self.prevTimeNs = mach_absolute_time();
    [NSTimer scheduledTimerWithTimeInterval:0 target:self selector:@selector(engineUpdate) userInfo:nil repeats:YES];
}


- (void)applicationWillTerminate:(NSNotification *)aNotification {
    // Insert code here to tear down your application
}


- (BOOL)applicationSupportsSecureRestorableState:(NSApplication *)app {
    return YES;
}

@end
