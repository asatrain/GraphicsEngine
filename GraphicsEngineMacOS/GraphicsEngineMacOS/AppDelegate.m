//
//  AppDelegate.m
//  GraphicsEngineMacOS
//
//  Created by Emin Asatrian on 19.07.2024.
//

#import "AppDelegate.h"
#import "GameView.h"

@implementation AppDelegate

- (void)applicationDidFinishLaunching:(NSNotification *)aNotification {
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
    
    GameView* gameView = [[GameView alloc] init];
    [self.window setContentView:gameView];
    [gameView startLoop];
}

@end
