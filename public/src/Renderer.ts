import * as PIXI from "pixi.js"
import {Provider} from "./provider/Provider";
import {PlayerView} from "./views/PlayerView";

export class Renderer {

    constructor(private provider: Provider) {}


    initialize(application: PIXI.Application, resources: any) {
        PlayerView.initialize(application, resources)

        let mainPlayerView: PlayerView

        this.provider.setOnTransfer(transfer => {
            PlayerView.hideAll()

            if (transfer.player) {
                mainPlayerView = PlayerView.create()
                mainPlayerView.setPosition(transfer.player.x, transfer.player.y)
                mainPlayerView.show()
            }

            if (transfer.players) {
                transfer.players.forEach(player => {
                    const view = PlayerView.create()
                    view.setPosition(player.x, player.y)
                    view.show()
                })
            }
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

        application.ticker.add(() => {
            if (this.provider.getStoredTransfer()) {
                if (keys.a) {
                    mainPlayerView.moveX(-this.provider.getStoredTransfer().player.speed)
                    this.provider.sendCommand("MoveLeft")
                }

                if (keys.d) {
                    mainPlayerView.moveX(this.provider.getStoredTransfer().player.speed)
                    this.provider.sendCommand("MoveRight")
                }

                if (keys.w) {
                    mainPlayerView.moveY(-this.provider.getStoredTransfer().player.speed)
                    this.provider.sendCommand("MoveUp")
                }

                if (keys.s) {
                    mainPlayerView.moveY(this.provider.getStoredTransfer().player.speed)
                    this.provider.sendCommand("MoveDown")
                }
            }
        })
    }

}