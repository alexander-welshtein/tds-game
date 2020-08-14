import * as PIXI from "pixi.js"
import {Provider} from "./provider/Provider";
import {PlayerEntity} from "./entities/PlayerEntity";

export class Renderer {

    constructor(private provider: Provider) {
    }


    initialize(application: PIXI.Application, resources: any) {
        PlayerEntity.initialize(application, resources)

        let entity: PlayerEntity

        this.provider.setOnTransfer(transfer => {
            PlayerEntity.hideAll()

            transfer.player && (entity = PlayerEntity.create(transfer.player.id)).applyTransfer(transfer.player)
            transfer.players && transfer.players.forEach(player => PlayerEntity.create(player.id).applyTransfer(player))
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

            if (this.provider.getStoredTransfer()) {
                if (keys.a) {
                    // entity.moveX(-this.provider.getStoredTransfer().player.speed)
                    this.provider.sendCommand("MoveLeft")
                }

                if (keys.d) {
                    // entity.moveX(this.provider.getStoredTransfer().player.speed)
                    this.provider.sendCommand("MoveRight")
                }

                if (keys.w) {
                    // entity.moveY(-this.provider.getStoredTransfer().player.speed)
                    this.provider.sendCommand("MoveUp")
                }

                if (keys.s) {
                    // entity.moveY(this.provider.getStoredTransfer().player.speed)
                    this.provider.sendCommand("MoveDown")
                }
            }
        })
    }

}