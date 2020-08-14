export class Pool<T> {

    private freeObjects: T[]
    private usedObjects: T[]


    constructor(private creator: () => T) {
        this.freeObjects = []
        this.usedObjects = []
    }


    get(): T {
        const object = this.freeObjects.pop() || this.creator()
        this.usedObjects.push(object)
        return object
    }

    put(object: T) {
        this.freeObjects.push(object)
        this.usedObjects.splice(this.usedObjects.indexOf(object), 1)
    }

    forEach(handler: (object: T, index?: number) => void) {
        this.usedObjects.forEach(item => handler(item))
    }

    reset() {
        this.freeObjects = [
            ...this.freeObjects,
            ...this.usedObjects
        ]
        this.usedObjects = []
    }
}