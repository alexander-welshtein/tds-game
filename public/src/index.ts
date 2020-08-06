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

    const socket = new WebSocket('ws://localhost:3000/ws/')

    socket.addEventListener('open', function() {
        this.send('Hello Server!')
    })

    socket.addEventListener('message', function(event) {
        const transfer = JSON.parse(event.data) as Transfer

        hull.x = transfer.player.x
        hull.y = transfer.player.y
    })

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
        keys.a && socket.readyState === socket.OPEN && socket.send('MoveLeft')
        keys.d && socket.readyState === socket.OPEN && socket.send('MoveRight')
        keys.w && socket.readyState === socket.OPEN && socket.send('MoveUp')
        keys.s && socket.readyState === socket.OPEN && socket.send('MoveDown')
    })
})