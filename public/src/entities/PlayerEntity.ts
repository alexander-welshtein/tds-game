import * as PIXI from "pixi.js"
import {Player, State} from "../provider/State";

export class PlayerEntity {

    private static entities: Map<string, PlayerEntity>
    private static creator: () => PlayerEntity

    private readonly sprite: PIXI.Sprite

    private resultX: number
    private resultY: number


    private constructor(application: PIXI.Application, resources: any) {
        this.sprite = new PIXI.Sprite(resources.hull.texture)
        application.stage.addChild(this.sprite)
    }


    static initialize(application: PIXI.Application, resources: any) {
        this.entities = new Map
        this.creator = () => new PlayerEntity(application, resources)
    }

    static create(id: string): PlayerEntity {
        if (this.entities.has(id)) {
            return this.entities.get(id)
        } else {
            const entity = this.creator()
            this.entities.set(id, entity)
            return entity
        }
    }

    static applyState(state: State) {
        this.entities.forEach(entity => entity.hide())
        state.player && PlayerEntity.create(state.player.id).applyState(state.player)
        state.players && state.players.forEach(player => PlayerEntity.create(player.id).applyState(player))
    }

    static update(deltaTime: number) {
        this.entities.forEach(entity => entity.update(deltaTime))
    }

    static resume() {
        this.entities.forEach(entity => entity.resume())
    }


    update(deltaTime: number) {
        const delta = deltaTime * 2

        if (this.sprite.x < this.resultX) {
            this.sprite.x += delta
            if (this.sprite.x > this.resultX - delta) {
                this.sprite.x = this.resultX
            }
        } else if (this.sprite.x > this.resultX) {
            this.sprite.x -= delta
            if (this.sprite.x < this.resultX + delta) {
                this.sprite.x = this.resultX
            }
        }

        if (this.sprite.y < this.resultY) {
            this.sprite.y += delta
            if (this.sprite.y > this.resultY - delta) {
                this.sprite.y = this.resultY
            }
        } else if (this.sprite.y > this.resultY) {
            this.sprite.y -= delta
            if (this.sprite.y < this.resultY + delta) {
                this.sprite.y = this.resultY
            }
        }
    }

    resume() {
        this.sprite.x = this.resultX
        this.sprite.y = this.resultY
    }

    applyState(player: Player) {
        this.resultX = player.x
        this.resultY = player.y
        this.sprite.visible = true
    }

    hide() {
        this.sprite.visible = false
    }

}