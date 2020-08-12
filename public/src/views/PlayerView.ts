import * as PIXI from "pixi.js"
import {Pool} from "../util/Pool";

export class PlayerView {

    private static pool: Pool<PlayerView>

    private readonly sprite: PIXI.Sprite


    private constructor(application: PIXI.Application, resources: any) {
        this.sprite = new PIXI.Sprite(resources.hull.texture)
        application.stage.addChild(this.sprite)
    }


    static initialize(application: PIXI.Application, resources: any) {
        this.pool = new Pool(() => new PlayerView(application, resources))
    }

    static create(): PlayerView {
        return this.pool.get()
    }

    static hideAll() {
        this.pool.forEachUsed(view => view.hide())
        this.pool.reset()
    }


    setPosition(x: number, y: number) {
        this.sprite.x = x
        this.sprite.y = y
    }

    moveX(x: number) {
        this.sprite.x += x
    }

    moveY(y: number) {
        this.sprite.y += y
    }

    show() {
        this.sprite.visible = true
    }

    hide() {
        this.sprite.visible = false
    }

}