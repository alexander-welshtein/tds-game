import {Operation, State} from "./State";
import {MathSet} from "../MathSet";
import {Command} from "./Command";

export class Provider {

    private socket: WebSocket
    private onState: (state: State) => void


    initialize() {
        this.socket = new WebSocket("ws://localhost:3000/ws/")

        this.socket.addEventListener("open", () => {
            this.sendCommand("JoinInstance")
        })

        this.socket.addEventListener("message", event => {
            const state = JSON.parse(event.data) as State
            this.onState && this.onState(state)
        })
    }

    setOnTransfer(onTransfer: (state: State) => void) {
        this.onState = onTransfer
    }

    sendCommand(command: keyof Command) {
        this.socket.readyState === this.socket.OPEN && this.socket.send(JSON.stringify({
            id: MathSet.randomNumberByMax(10000),
            command
        } as Operation))
    }
}