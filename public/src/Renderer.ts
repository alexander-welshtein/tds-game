import * as PIXI from "pixi.js"
import {Provider} from "./provider/Provider";
import {PlayerEntity} from "./entities/PlayerEntity";

export class Renderer {

    constructor(private provider: Provider) {
    }


    initialize(application: PIXI.Application, resources: any) {
        PlayerEntity.initialize(application, resources)

        let entity: PlayerEntity

        this.provider.setOnTransfer(state => {
            PlayerEntity.hideAll()

            state.player && (entity = PlayerEntity.create(state.player.id)).applyTransfer(state.player)
            state.players && state.players.forEach(player => PlayerEntity.create(player.id).applyTransfer(player))
        })

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

        application.ticker.add(deltaTime => {
            PlayerEntity.updateAll(deltaTime)
        })
    }

}