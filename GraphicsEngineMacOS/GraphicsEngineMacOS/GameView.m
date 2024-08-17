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
    self.prevTimeNs = clock_gettime_nsec_np(CLOCK_MONOTONIC_RAW);
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
    
    uint64_t currentTimeNs = clock_gettime_nsec_np(CLOCK_MONOTONIC_RAW);
    double deltaTime = (currentTimeNs - self.prevTimeNs) / 1000000000.0;
//    NSLog(@"%f ms deltaTime, FPS: %i", deltaTime, (int)(1 / deltaTime));
    self.prevTimeNs = currentTimeNs;
    
    int32_t width = self.bounds.size.width;
    int32_t height = self.bounds.size.height;
    
    free_bitmap(self.bitmap, width * height);
    self.bitmap = update_and_render(width, height, input, deltaTime);
    
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
//            NSLog(@"W is pressed");
	    input.w_pressed = YES;
            break;
        case 0x00:
//            NSLog(@"A is pressed");
	    input.a_pressed = YES;
            break;
        case 0x01:
//            NSLog(@"S is pressed");
	    input.s_pressed = YES;
            break;
        case 0x02:
//            NSLog(@"D is pressed");
	    input.d_pressed = YES;
            break;
        case 0x0C:
//            NSLog(@"Q is pressed");
        input.q_pressed = YES;
            break;
        case 0x0E:
//            NSLog(@"E is pressed");
        input.e_pressed = YES;
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
//            NSLog(@"W released");
            input.w_pressed = NO;
            break;
        case 0x00:
//            NSLog(@"A released");
            input.a_pressed = NO;
            break;
        case 0x01:
//            NSLog(@"S released");
            input.s_pressed = NO;
            break;
        case 0x02:
//            NSLog(@"D released");
            input.d_pressed = NO;
            break;
        case 0x0C:
//            NSLog(@"Q is released");
            input.q_pressed = NO;
            break;
        case 0x0E:
//            NSLog(@"E is released");
            input.e_pressed = NO;
            break;
        default:
            NSLog(@"Not handled key code: %hu", keyCode);
            break;
    }
}

- (void)flagsChanged:(NSEvent *)event {
    if ([event modifierFlags] & NSEventModifierFlagShift) {
//        NSLog(@"Shift is pressed");
        input.shift_pressed = YES;
    } else {
//        NSLog(@"Shift is released");
        input.shift_pressed = NO;
    }
}

@end
