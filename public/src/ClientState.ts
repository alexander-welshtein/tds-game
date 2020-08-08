export interface ClientState {
    operation: Operation
    player: {
        x: number,
        y: number,
        speed: number
    }
}

export class Operation {
    id: number
    command: string
}
