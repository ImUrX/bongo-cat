const iohook = require("iohook");
const config = require("./config.json");
const triggers = new Map(Object.entries(config.triggers));

function someModifiersTrue(event) {
    return event.altKey || event.shiftKey || event.ctrlKey || event.metaKey;
}

document.body.style.backgroundColor = `rgb(${config.chroma_key.replace(/ /g, ",")})`;

for(const [key, value] of Object.entries(config.images)) {
    document.getElementById(`${key}y-cat`).src = value;
}

let activeKey;

iohook.on("keydown", event => {
    if(someModifiersTrue(event) || activeKey == event.keycode || !triggers.has(event.keycode.toString())) return;
    if(activeKey) {
        const active = triggers.get(activeKey);
        document.getElementById(`${active}y-cat`).style.visibility = "hidden";
    } else {
        document.getElementById("neutraly-cat").style.visibility = "hidden";
    }
    activeKey = event.keycode.toString();
    const active = triggers.get(activeKey);
    document.getElementById(`${active}y-cat`).style.visibility = "visible";
});

iohook.on("keyup", event => {
    if(someModifiersTrue(event) || activeKey != event.keycode) return;
    const active = triggers.get(activeKey);
    document.getElementById(`${active}y-cat`).style.visibility = "hidden";
    document.getElementById("neutraly-cat").style.visibility = "visible";
    activeKey = null;
});

iohook.start();

document.getElementById("neutraly-cat").style.visibility = "visible";