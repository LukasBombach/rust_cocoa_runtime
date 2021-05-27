
#[macro_use] extern crate cocoa;
#[macro_use] extern crate objc;

use objc::runtime::{Object, Sel};

use cocoa::base::{ nil, id, YES, NO};

use cocoa::foundation::{NSRect, NSPoint, NSSize, NSAutoreleasePool, NSString,
                        NSDefaultRunLoopMode};

use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivationPolicyRegular, 
                    NSBackingStoreBuffered, NSWindowStyleMask, NSRunningApplication,
                    NSApplicationActivateIgnoringOtherApps, NSWindow};

use neon::prelude::*;


fn open_window(mut cx: FunctionContext) -> JsResult<JsString> {

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

    Ok(cx.string("return from rust"))
}

fn send_os_events(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    unsafe {
        let app = NSApp();
        let ns_event_mask_any = !0;

        loop {
            let event = app.nextEventMatchingMask_untilDate_inMode_dequeue_(ns_event_mask_any, nil, NSDefaultRunLoopMode, YES);

            if event == nil {
                break;
            }

            let () = msg_send![app, sendEvent:event];

        }
    }

    Ok(cx.undefined())
}
#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("open_window", open_window)?;
    cx.export_function("send_os_events", send_os_events)?;
    Ok(())
}
