#[macro_use] extern crate cocoa;
#[macro_use] extern crate objc;

use objc::runtime::{Object, Sel};

use cocoa::base::{ nil, id, NO};

use cocoa::foundation::{NSRect, NSPoint, NSSize, NSAutoreleasePool, NSString};
use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivationPolicyRegular, 
                    NSBackingStoreBuffered, NSWindowStyleMask, NSRunningApplication,
                    NSApplicationActivateIgnoringOtherApps, NSWindow};

fn main() {
    unsafe {
        let _pool = NSAutoreleasePool::new(nil);

        // app
        let app = NSApp();
        app.setActivationPolicy_(NSApplicationActivationPolicyRegular);

        // app delegate
        let () = msg_send![app, setDelegate: delegate!("MyAppDelegate", {
            (applicationDidFinishLaunching:) => app_did_finish_launching as extern "C" fn(&Object, Sel, id)
        })];

        extern "C" fn app_did_finish_launching(_: &Object, _: Sel, _: id) {
            println!("application_did_finish_launching");
            unsafe {
                let app = NSApp();
                let () = msg_send![app, stop: nil];
            }
        }

        // create Window
        let window = NSWindow::alloc(nil)
            .initWithContentRect_styleMask_backing_defer_(
                NSRect::new(NSPoint::new(0., 0.), NSSize::new(200., 200.)),
                NSWindowStyleMask::NSTitledWindowMask,
                NSBackingStoreBuffered,
                NO
            )
            .autorelease();

        window.cascadeTopLeftFromPoint_(NSPoint::new(20., 20.));
        window.center();
        window.setTitle_(NSString::alloc(nil).init_str("Hello World!"));
        window.makeKeyAndOrderFront_(nil);

        let current_app = NSRunningApplication::currentApplication(nil);
        current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);

        // run the app
        app.run();
    }
}