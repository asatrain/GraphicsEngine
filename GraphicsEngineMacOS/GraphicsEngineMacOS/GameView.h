//
//  GameView.h
//  GraphicsEngineMacOS
//
//  Created by Emin Asatrian on 21.07.2024.
//

#import <Cocoa/Cocoa.h>

typedef struct {
    uint8_t red;
    uint8_t green;
    uint8_t blue;
    uint8_t alpha;
} Pixel;

@interface GameView : NSView

@property NSTimer *gameLoopTimer;
@property uint64_t prevTimeNs;
@property Pixel* buffer;

- (void)startLoop;

@end
