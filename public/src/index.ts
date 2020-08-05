import * as PIXI from "pixi.js"
import "assets/hull.png"

const app = new PIXI.Application()

document.body.appendChild(app.view)

app.loader.add([
    {
        name: 'hull',
        url: 'hull.png'
    }
]).load((loader, resources) => {
    const hull = new PIXI.Sprite(resources.hull.texture)

    hull.x = app.renderer.width / 2
    hull.y = app.renderer.height / 2

    hull.anchor.x = .5
    hull.anchor.y = .5

    app.stage.addChild(hull)

    const keys = {
        a: false,
        d: false,
        w: false,
        s: false
    }

    Object.keys(keys).forEach(key => {
        window.addEventListener('keydown', e => e.key == key && (keys[key] = true))
        window.addEventListener('keyup', e => e.key == key && (keys[key] = false))
    })

    app.ticker.add(() => {
        keys.a && (hull.x -= 5)
        keys.d && (hull.x += 5)
        keys.w && (hull.y -= 5)
        keys.s && (hull.y += 5)
    })
})