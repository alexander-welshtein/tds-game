import * as PIXI from "pixi.js"
import {Provider} from "./provider/Provider";
import {PlayerEntity} from "./entities/PlayerEntity";
import {State} from "./provider/State";

export class Renderer {

    constructor(private provider: Provider) {
    }


    initialize(application: PIXI.Application, resources: any) {
        PlayerEntity.initialize(application, resources)

        const states: State[] = []

        this.provider.setOnState(state => states.push(state))

        const keysState = {
            a: false,
            d: false,
            w: false,
            s: false
        }

        const keysCommands = {
            a: "KeyLeft",
            d: "KeyRight",
            w: "KeyUp",
            s: "KeyDown"
        }

        window.addEventListener('keydown', e => {
            if (!keysState[e.key]) {
                keysState[e.key] = true
                this.provider.sendCommand(`${keysCommands[e.key]}Down` as any)
            }
        })

        window.addEventListener('keyup', e => {
            if (keysState[e.key]) {
                keysState[e.key] = false
                this.provider.sendCommand(`${keysCommands[e.key]}Up` as any)
            }
        })

        document.addEventListener('visibilitychange', () => Object.keys(keysState).forEach(key => {
            keysState[key] = false;
            this.provider.sendCommand(`${keysCommands[key]}Up` as any)
        }))

        application.ticker.add(() => {
            const state = states.pop()
            states.length = 0
            state && PlayerEntity.applyState(state)
        })

        application.ticker.add(deltaTime => {
            PlayerEntity.update(deltaTime)
            states.length = 0
        })
    }

}