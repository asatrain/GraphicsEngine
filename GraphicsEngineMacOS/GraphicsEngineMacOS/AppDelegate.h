//
//  AppDelegate.h
//  GraphicsEngineMacOS
//
//  Created by Emin Asatrian on 19.07.2024.
//

#import <Cocoa/Cocoa.h>

typedef struct {
    uint8_t red;
    uint8_t green;
    uint8_t blue;
    uint8_t alpha;
} Pixel;

@interface AppDelegate : NSObject <NSApplicationDelegate>

@end

