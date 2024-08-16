//
//  GameView.m
//  GraphicsEngineMacOS
//
//  Created by Emin Asatrian on 21.07.2024.
//

#import "GameView.h"

@implementation GameView

UserInput input;

- (void)startLoop {
    create_scene();
    self.bitmap = NULL;
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
//    NSLog(@"%f ms deltaTime", deltaTimeMs);
    self.prevTimeNs = currentTimeNs;
    
    int32_t width = self.bounds.size.width;
    int32_t height = self.bounds.size.height;
    
    free_bitmap(self.bitmap, width * height);
    self.bitmap = update_and_render(width, height, input, deltaTimeMs);
    
    CGContextRef gContext = [[NSGraphicsContext currentContext] CGContext];
    
    CGColorSpaceRef colorSpace = CGColorSpaceCreateDeviceRGB();
    CGContextRef imgContext = CGBitmapContextCreate(self.bitmap,
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

- (BOOL)acceptsFirstResponder {
    return YES;
}

- (void)mouseDown:(NSEvent*)event {
    NSPoint location = [self convertPoint:[event locationInWindow] fromView:nil];
    NSLog(@"Mouse down at: %@", NSStringFromPoint(location));
}

- (void)keyDown:(NSEvent*)event {
    unsigned short keyCode = [event keyCode];
    switch (keyCode) {
        case 0x0D:
            NSLog(@"W is pressed");
	    input.w_pressed = YES;
            break;
        case 0x00:
            NSLog(@"A is pressed");
	    input.a_pressed = YES;
            break;
        case 0x01:
            NSLog(@"S is pressed");
	    input.s_pressed = YES;
            break;
        case 0x02:
            NSLog(@"D is pressed");
	    input.d_pressed = YES;
            break;
        default:
            NSLog(@"Not handled key code: %hu", keyCode);
            break;
    }
}

- (void)keyUp:(NSEvent*)event {
    unsigned short keyCode = [event keyCode];
    switch (keyCode) {
        case 0x0D:
            NSLog(@"W key released");
	    input.w_pressed = NO;
            break;
        case 0x00:
            NSLog(@"A key released");
	    input.a_pressed = NO;
            break;
        case 0x01:
            NSLog(@"S key released");
	    input.s_pressed = NO;
            break;
        case 0x02:
            NSLog(@"D key pressed");
	    input.d_pressed = NO;
            break;
        default:
            NSLog(@"Not handled key code: %hu", keyCode);
            break;
    }
}

@end
