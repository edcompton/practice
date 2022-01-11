type NewMixinConstructor<T> = new(...args: any[]) => T;

interface shapeClassesUsedMustSatisfy {
    toMultiply(): Array<number>
}

function myMultiplicationMixin<C extends NewMixinConstructor<shapeClassesUsedMustSatisfy>>(Class: C) {
    return class extends Class {
        multiply() {
            const nameOfClass = this.constructor.name;
            const values = this.toMultiply();
            console.log(nameOfClass, values.reduce((value: number, acc: number) => {return value * acc}, 1))
        }
    }
}

class MyMultiplicationClass {
    constructor(private firstNum: number,
                private secondNum: number,
                private thirdNum: number) {}
    toMultiply() {
        return [this.firstNum, this.secondNum, this.thirdNum];
    }
}

const MultiplicationClass = myMultiplicationMixin(MyMultiplicationClass)

const multiplicationInstance = new MultiplicationClass(1,2,3);

multiplicationInstance.multiply();

type Payload = {};

@serializable
class APIPayload {
    getValue(): Payload {
        return {}
    }
}

type ClassConstructor<T> = new(...args: any[]) => T;

function serializable<T extends ClassConstructor<{getValue(): Payload}>>(Constructor: T) {
    return class extends Constructor {
        serialize() {
            this.getValue().toString();
        }
    }
}

const apiClass = new APIPayload;

// @ts-ignore
console.log(apiClass.serialize())
