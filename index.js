const { app, BrowserWindow } = require("electron");
const fs = require("fs");
const path = require("path");

let win;

function createWindow() {

    win = new BrowserWindow({
        webPreferences: {
            nodeIntegration: true
        }
    });
    fs.access("./config.json", (err) => {
        if(err) fs.copyFileSync(path.join(__dirname, "config.json.sample"), path.join(__dirname, "config.json"));
        const config = require("./config.json");
        if(config.size && config.size.width && config.size.height) {
            win.setBounds({ width: config.size.width, height: config.size.height });
        }
    });

    win.loadFile("index.html");

    win.removeMenu();
  
    win.on("closed", () => win = null);
}

app.on("ready", createWindow);

app.on("window-all-closed", () => {
    if (process.platform !== "darwin") app.quit();
});

app.on("activate", () => {
    if (win === null) createWindow();
});
