export interface Transfer {
    session_id: number,
    player: {
        x: number,
        y: number,
        speed: number,
        operation: Operation
    }
}

export class Operation {
    id: number
    command: string
}
