import {Command} from "./Command";

export interface State {
    player: Player
    players: Player[]
}

export interface Operation {
    id: number
    command: keyof Command
}

export interface Player {
    id: string
    x: number
    y: number
    speed: number
}