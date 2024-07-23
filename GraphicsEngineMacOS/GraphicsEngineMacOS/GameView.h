//
//  GameView.h
//  GraphicsEngineMacOS
//
//  Created by Emin Asatrian on 21.07.2024.
//

#import <Cocoa/Cocoa.h>
#import "../../GraphicsEngineLib/src/graphics_engine.h"

@interface GameView : NSView

@property NSTimer *gameLoopTimer;
@property uint64_t prevTimeNs;
@property Pixel* buffer;

- (void)startLoop;

@end
