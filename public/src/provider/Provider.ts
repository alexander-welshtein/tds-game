import {Operation, Transfer} from "./Transfer";
import {MathSet} from "../MathSet";
import {Command} from "./Command";

export class Provider {

    private socket: WebSocket
    private onTransfer: (transfer: Transfer) => void
    private storedTransfer: Transfer


    initialize() {
        this.socket = new WebSocket("ws://localhost:3000/ws/")

        this.socket.addEventListener("open", () => {
            this.sendCommand("JoinInstance")
        })

        this.socket.addEventListener("message", event => {
            const transfer = JSON.parse(event.data) as Transfer
            this.onTransfer && (!this.storedTransfer || transfer.player.operation.id == this.storedTransfer.player.operation.id) && this.onTransfer(transfer)
            this.storedTransfer = transfer
        })
    }

    setOnTransfer(onTransfer: (transfer: Transfer) => void) {
        this.onTransfer = onTransfer
    }

    getStoredTransfer() {
        return this.storedTransfer
    }


    sendCommand(command: keyof Command) {
        this.socket.readyState === this.socket.OPEN && this.socket.send(JSON.stringify({
            id: MathSet.randomNumberByMax(10000),
            command
        } as Operation))
    }
}