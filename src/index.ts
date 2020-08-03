import * as PIXI from "pixi.js"
import "assets/hull.png"

const app = new PIXI.Application()

document.body.appendChild(app.view)

app.loader.add('hull', 'hull.png').load((loader, resources) => {
    const hull = new PIXI.Sprite(resources.hull.texture)

    hull.x = app.renderer.width / 2
    hull.y = app.renderer.height / 2

    hull.anchor.x = .5
    hull.anchor.y = .5

    app.stage.addChild(hull)

    app.ticker.add(() => {
        hull.rotation += .01
    })
})