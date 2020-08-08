import * as PIXI from "pixi.js"
import {ClientState, Operation} from "./ClientState";
import {MathSet} from "./MathSet";

export function Client(config: {
    app: PIXI.Application
    hullTexture: PIXI.Texture
}) {
    const hull = new PIXI.Sprite(config.hullTexture)

    hull.anchor.x = .5
    hull.anchor.y = .5

    config.app.stage.addChild(hull)

    let state: ClientState

    const socket = new WebSocket('ws://localhost:3000/ws/')

    socket.addEventListener('open', function () {
        this.send('Hello Server!')
    })

    socket.addEventListener('message', function (event) {
        state = JSON.parse(event.data) as ClientState

       if (state.operation.id == lastOperation.id) {
           hull.x = state.player.x
           hull.y = state.player.y
       }
    })

    let lastOperation: Operation

    const send = (command: string) => {
        socket.send(JSON.stringify(lastOperation = {
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
        if (keys.a) {
            hull.x -= state.player.speed
            socket.readyState === socket.OPEN && send('MoveLeft')
        }

        if (keys.d) {
            hull.x += state.player.speed
            socket.readyState === socket.OPEN && send('MoveRight')
        }

        if (keys.w) {
            hull.y -= state.player.speed
            socket.readyState === socket.OPEN && send('MoveUp')
        }

        if (keys.s) {
            hull.y += state.player.speed
            socket.readyState === socket.OPEN && send('MoveDown')
        }
    })
}