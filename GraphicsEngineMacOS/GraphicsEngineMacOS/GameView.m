//
//  GameView.m
//  GraphicsEngineMacOS
//
//  Created by Emin Asatrian on 21.07.2024.
//

#import "GameView.h"

@implementation GameView

- (void)startLoop {
    self.buffer = NULL;
    self.prevTimeNs = mach_absolute_time();
    self.gameLoopTimer = [NSTimer scheduledTimerWithTimeInterval:0
                                                          target:self
                                                        selector:@selector(triggerDraw)
                                                        userInfo:nil
                                                         repeats:YES];
}

- (void)triggerDraw {
    [self setNeedsDisplay:YES];
}

- (void)drawRect:(NSRect)dirtyRect {
    [super drawRect:dirtyRect];
    
    uint64_t currentTimeNs = mach_absolute_time();
    uint64_t deltaTimeNs = currentTimeNs - self.prevTimeNs;
    NSLog(@"%f ms deltaTime", deltaTimeNs / 1000000.0f);
    self.prevTimeNs = currentTimeNs;
    
    size_t width = self.bounds.size.width;
    size_t height = self.bounds.size.height;
    
    free(self.buffer);
    self.buffer = calloc(width * height, sizeof(Pixel));
    
    Pixel red = {.red=200, .green=0, .blue=0, .alpha=125};
    for (size_t i = 0; i < height; ++i) {
        for (size_t j = 0; j < width; ++j) {
            self.buffer[i * width + j] = red;
        }
    }
    
    Pixel green = {.red=0, .green=200, .blue=0, .alpha=255};
    for (size_t i = 100; i < 200; ++i) {
        for (size_t j = 150; j < 200; ++j) {
            self.buffer[i * width + j] = green;
        }
    }
    
    CGContextRef gContext = [[NSGraphicsContext currentContext] CGContext];
    
    CGColorSpaceRef colorSpace = CGColorSpaceCreateDeviceRGB();
    CGContextRef imgContext = CGBitmapContextCreate(self.buffer,
                                                      width,
                                                      height,
                                                      8,
                                                      width * sizeof(Pixel),
                                                      colorSpace,
                                                      kCGImageAlphaNoneSkipLast);
    CGImageRef imgRef = CGBitmapContextCreateImage(imgContext);
    
    CGContextDrawImage(gContext, CGRectMake(0, 0, width, height), imgRef);
    
    CGImageRelease(imgRef);
    CGContextRelease(imgContext);
    CGColorSpaceRelease(colorSpace);
}

@end
