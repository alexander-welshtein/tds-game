import * as PIXI from "pixi.js"
import {Provider} from "./provider/Provider";
import {Renderer} from "./Renderer";

const application = new PIXI.Application()

document.body.appendChild(application.view)

application.loader.add([
    {
        name: 'player_walk',
        url: 'player_walk.json'
    }
]).load((loader, resources) => {

    const provider = new Provider()
    provider.initialize()

    const renderer = new Renderer(provider)
    renderer.initialize(application, resources)
})