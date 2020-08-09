import * as PIXI from "pixi.js"
import {Transfer, Operation} from "./Transfer";
import {MathSet} from "./MathSet";

export function Client(config: {
    app: PIXI.Application
    hullTexture: PIXI.Texture
}) {
    const hull = new PIXI.Sprite(config.hullTexture)

    hull.anchor.x = .5
    hull.anchor.y = .5

    config.app.stage.addChild(hull)

    let stored_transfer: Transfer

    const socket = new WebSocket('ws://localhost:3000/ws/')

    socket.addEventListener('open', function () {
        send('JoinInstance')
    })

    socket.addEventListener('message', function (event) {
        const transfer = JSON.parse(event.data) as Transfer

       if (!stored_transfer || transfer.player.operation.id == stored_transfer.player.operation.id) {
           hull.x = transfer.player.x
           hull.y = transfer.player.y
       }

       stored_transfer = transfer
    })

    const send = (command: string) => {
        socket.send(JSON.stringify({
            id: MathSet.randomNumberByMax(10000),
            command
        } as Operation))
    }

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

    config.app.ticker.add(() => {
        if (stored_transfer) {
            if (keys.a) {
                hull.x -= stored_transfer.player.speed
                socket.readyState === socket.OPEN && send('MoveLeft')
            }

            if (keys.d) {
                hull.x += stored_transfer.player.speed
                socket.readyState === socket.OPEN && send('MoveRight')
            }

            if (keys.w) {
                hull.y -= stored_transfer.player.speed
                socket.readyState === socket.OPEN && send('MoveUp')
            }

            if (keys.s) {
                hull.y += stored_transfer.player.speed
                socket.readyState === socket.OPEN && send('MoveDown')
            }
        }
    })
}