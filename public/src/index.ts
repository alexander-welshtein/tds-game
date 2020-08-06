import * as PIXI from "pixi.js"
import "assets/hull.png"
import {Client} from "./Client";

const app = new PIXI.Application()

document.body.appendChild(app.view)

app.loader.add([
    {
        name: 'hull',
        url: 'hull.png'
    }
]).load((loader, resources) => {
    Client({
        app,
        hullTexture: resources.hull.texture
    })
})