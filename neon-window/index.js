const addon = require(".");

function sendEventsOnNextTick() {
  process.nextTick(() => {
    addon.send_os_events();
    sendEventsOnNextTick();
  });
}

const rustReturnValue = addon.open_window();

console.log(rustReturnValue);

sendEventsOnNextTick();

console.log("Code executed after non-breaking loop");
