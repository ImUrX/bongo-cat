const { app, BrowserWindow, Menu, MenuItem } = require("electron");
const Store = require("electron-store");
const schema = require("./schema.json");

let win;

const store = new Store({ schema });

if(!store.get("hardware_acceleration")) {
    app.disableHardwareAcceleration();
}

const menu = new Menu();

menu.append(new MenuItem({
    label: "File",
    role: "fileMenu"
}));

menu.append(new MenuItem({
    label: "View",
    role: "viewMenu"
}));

menu.append(new MenuItem({
    label: "Window",
    role: "windowMenu"
}));

menu.append(new MenuItem({
    label: "Configure",
    accelerator: process.platform === "darwin" ? "Alt+Cmd+O" : "Ctrl+Shift+O",
    click: (event, focusedWin) => focusedWin.loadFile("config.html")
}));

function createWindow() {

    win = new BrowserWindow({
        width: store.get("size.width"),
        height: store.get("size.height"),
        autoHideMenuBar: store.get("auto_hide_menu_bar"),
        webPreferences: {
            nodeIntegration: true
        }
    });

    win.setMenu(menu);

    win.loadFile("index.html");

    win.on("closed", () => win = null);
}

app.on("ready", createWindow);

app.on("window-all-closed", () => {
    if (process.platform !== "darwin") app.quit();
});

app.on("activate", () => {
    if (win === null) createWindow();
});
