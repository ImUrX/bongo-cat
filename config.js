const Store = require("electron-store");
const schema = require("./schema.json");

const store = new Store({ schema });

store;