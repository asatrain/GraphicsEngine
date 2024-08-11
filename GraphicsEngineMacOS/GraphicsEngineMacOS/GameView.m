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
    float deltaTimeMs = (currentTimeNs - self.prevTimeNs) / 1000000.0f;
    NSLog(@"%f ms deltaTime", deltaTimeMs);
    self.prevTimeNs = currentTimeNs;
    
    size_t width = self.bounds.size.width;
    size_t height = self.bounds.size.height;
    
    free_buffer(self.buffer, width * height);
    self.buffer = update_and_render(width, height, deltaTimeMs);
    
    CGContextRef gContext = [[NSGraphicsContext currentContext] CGContext];
    
    CGColorSpaceRef colorSpace = CGColorSpaceCreateDeviceRGB();
    CGContextRef imgContext = CGBitmapContextCreate(self.buffer,
                                                      width,
                                                      height,
                                                      8,
                                                      width * sizeof(Color),
                                                      colorSpace,
                                                      kCGImageAlphaNoneSkipLast);
    CGImageRef imgRef = CGBitmapContextCreateImage(imgContext);
    
    CGContextDrawImage(gContext, CGRectMake(0, 0, width, height), imgRef);
    
    CGImageRelease(imgRef);
    CGContextRelease(imgContext);
    CGColorSpaceRelease(colorSpace);
}

@end
