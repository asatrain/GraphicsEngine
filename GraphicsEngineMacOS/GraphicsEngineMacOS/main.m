//
//  main.m
//  GraphicsEngineMacOS
//
//  Created by Emin Asatrian on 19.07.2024.
//

#import <Cocoa/Cocoa.h>
#import "AppDelegate.h"

int main(int argc, const char * argv[]) {
    NSApplication* app = [NSApplication sharedApplication];
    AppDelegate* appDelegate = [[AppDelegate alloc] init];
    app.delegate = appDelegate;
    
    return NSApplicationMain(argc, argv);
}
