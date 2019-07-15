const iohook = require("iohook");
const Store = require("electron-store");
const schema = require("./schema.json");

const store = new Store({ schema });

function someModifiersTrue(event) {
    return event.altKey || event.shiftKey || event.ctrlKey || event.metaKey;
}

function getMode(keycode) {
    for(const [key, value] of Object.entries(store.get("triggers"))) {
        if(value.some(val => val === keycode)) return key;
    }
}

document.body.style.backgroundColor = `rgb(${store.get("chroma_key").join(",")})`;

for(const [key, value] of Object.entries(store.get("images"))) {
    document.getElementById(`${key}y-cat`).src = value;
}

let activeMode;
document.getElementById("neutraly-cat").style.visibility = "visible";

iohook.on("keydown", event => {
    if(someModifiersTrue(event)) return;
    const mode = getMode(event.keycode);
    if(mode === activeMode || !mode) return;
    if(activeMode) {
        document.getElementById(`${activeMode}y-cat`).style.visibility = "hidden";
    } else {
        document.getElementById("neutraly-cat").style.visibility = "hidden";
        activeMode = mode;
        document.getElementById(`${activeMode}y-cat`).style.visibility = "visible";
        return;
    }
    activeMode = "both";
    document.getElementById(`${activeMode}y-cat`).style.visibility = "visible";
});

iohook.on("keyup", event => {
    const mode = getMode(event.keycode);
    if(someModifiersTrue(event) || (!mode && activeMode === "both")) return;
    document.getElementById(`${activeMode}y-cat`).style.visibility = "hidden";
    if(activeMode === "both") {
        if(mode === "left") {
            activeMode = "right";
            document.getElementById("righty-cat").style.visibility = "visible";
        } else {
            activeMode = "left";
            document.getElementById("lefty-cat").style.visibility = "visible";
        }
        return;
    }
    document.getElementById("neutraly-cat").style.visibility = "visible";
    activeMode = null;
});

iohook.start();
