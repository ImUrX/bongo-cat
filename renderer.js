const iohook = require("iohook");
const Store = require("electron-store");
const schema = require("./schema.json");
const fs = require("fs");

const store = new Store({ schema });
const modes = {
    left: -1,
    right: -1
};
const keycodeNames = {
    0: "§",
    1: "Escape",
    2: "1",
    3: "2",
    4: "3",
    5: "4",
    6: "5",
    7: "6",
    8: "7",
    9: "8",
    10: "9",
    11: "0",
    12: "-",
    13: "=",
    14: "Backspace",
    15: "Tab",
    16: "q",
    17: "w",
    18: "e",
    19: "r",
    20: "t",
    21: "y",
    22: "u",
    23: "i",
    24: "o",
    25: "p",
    26: "[",
    27: "]",
    28: "Enter",
    29: "Control",
    30: "a",
    31: "s",
    32: "d",
    33: "f",
    34: "g",
    35: "h",
    36: "j",
    37: "k",
    38: "l",
    39: ";",
    40: "'",
    41: "`",
    42: "Left Shift",
    43: "\\",
    44: "z",
    45: "x",
    46: "c",
    47: "v",
    48: "b",
    49: "n",
    50: "m",
    51: ",",
    52: ".",
    53: "/",
    54: "Right Shift",
    56: "Left Alt",
    57: "Space",
    58: "CapsLock",
    3640: "Right Alt",
    3675: "Left Command",
    3676: "Right Command"
};
let osu = false;

{
    const osuConfigFile = store.get("osu_config_file");
    if(osuConfigFile != null) {
        const file = fs.readFileSync(osuConfigFile, "utf8");
        const arr = [...Object.entries(keycodeNames)];
        osu = true;
        const [,left] = /keyOsuLeft = (\w)/.exec(file);
        const [,right] = /keyOsuRight = (\w)/.exec(file);
        modes.left = arr.find(code => left.toLowerCase() === code[1])[0];
        modes.right = arr.find(code => right.toLowerCase() === code[1])[0];
        console.log(modes);
    }
}

function someModifiersTrue(event) {
    return event.altKey || event.shiftKey || event.ctrlKey || event.metaKey;
}

function getMode(keycode) {
    if(osu) {
        if(modes.right == keycode) return "right";
        if(modes.left == keycode) return "left";
    } else {
        for(const [key, value] of Object.entries(store.get("triggers"))) {
            if(value.some(val => val === keycode)) return key;
        }
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
    if(someModifiersTrue(event) || (!mode && activeMode === "both") || !activeMode) return;
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
