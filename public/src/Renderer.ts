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

        window.addEventListener('keydown', event => {
            if (!keysState[event.key]) {
                keysState[event.key] = true
                this.provider.sendCommand(`${keysCommands[event.key]}Down` as any)
            }
        })

        window.addEventListener('keyup', event => {
            if (keysState[event.key]) {
                keysState[event.key] = false
                this.provider.sendCommand(`${keysCommands[event.key]}Up` as any)
            }
        })

        let resumed = true

        document.addEventListener('visibilitychange', () => Object.keys(keysState).forEach(key => {
            if (document.hidden) {
                keysState[key] = false;
                this.provider.sendCommand(`${keysCommands[key]}Up` as any)
            } else {
                resumed = true
            }
        }))

        application.ticker.add(() => {
            const state = states.pop()
            states.length = 0

            if (state) {
                PlayerEntity.applyState(state)

                if (resumed) {
                    PlayerEntity.resume()
                    resumed = false
                }
            }
        })

        application.ticker.add(deltaTime => {
            !resumed && PlayerEntity.update(deltaTime)
        })
    }

}