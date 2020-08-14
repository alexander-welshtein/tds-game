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

        application.ticker.add(deltaTime => {
            PlayerEntity.updateAll(deltaTime)

            if (keys.a) {
                this.provider.sendCommand("MoveLeft")
            }

            if (keys.d) {
                this.provider.sendCommand("MoveRight")
            }

            if (keys.w) {
                this.provider.sendCommand("MoveUp")
            }

            if (keys.s) {
                this.provider.sendCommand("MoveDown")
            }
        })
    }

}