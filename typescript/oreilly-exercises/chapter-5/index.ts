// Mixins Exercise
type NewMixinConstructor<T> = new(...args: any[]) => T;

interface shapeClassesUsedMustSatisfy {
    toMultiply(): Array<number>
}

function myMultiplicationMixin<C extends NewMixinConstructor<shapeClassesUsedMustSatisfy>>(Class: C) {
    return class extends Class {
        multiply() {
            const nameOfClass = this.constructor.name;
            const values = this.toMultiply();
            console.log(nameOfClass, values.reduce((value: number, acc: number) => {
                return value * acc
            }, 1))
        }
    }
}

class MyMultiplicationClass {
    constructor(private firstNum: number,
                private secondNum: number,
                private thirdNum: number) {
    }

    toMultiply() {
        return [this.firstNum, this.secondNum, this.thirdNum];
    }
}

const MultiplicationClass = myMultiplicationMixin(MyMultiplicationClass)

const multiplicationInstance = new MultiplicationClass(1, 2, 3);

multiplicationInstance.multiply();

type Payload = {};

@serializable
class APIPayload {
    getValue(): Payload {
        return {}
    }
}

type ClassConstructor<T> = new(...args: any[]) => T;

function serializable<T extends ClassConstructor<{ getValue(): Payload }>>(Constructor: T) {
    return class extends Constructor {
        serialize() {
            this.getValue().toString();
        }
    }
}

const apiClass = new APIPayload;

// @ts-ignore
console.log(apiClass.serialize())

// Exercise question 2: When you mark a class’s constructor as private, that means you can’t instantiate or extend the class.
// What happens when you mark it as protected instead? Play around with this in your code editor, and see if you can figure it out.

class ProtectedClass {
    readonly name: string = '';

    protected constructor(name: string) {
        this.name = name;
        console.log(this.name);
    }

    public create(name: string) {
        return new ProtectedClass(name);
    }
}

class NonProtectedClass extends ProtectedClass {
    constructor(name: string) {
        super(name);
    }
}

new NonProtectedClass('Ed');

// Question 3: “Extend the implementation we developed “Factory Pattern” to make it safer, at the expense of breaking the abstraction a bit.
// Update the implementation so that a consumer knows at compile time that calling Shoe.create('boot') returns a Boot
// and calling Shoe.create('balletFlat') returns a BalletFlat (rather than both returning a Shoe).
// Hint: think back to “Overloaded Function Types”.

type Shoe = {
    purpose: string
}

class BalletFlat implements Shoe {
    purpose = 'dancing'
}

class Boot implements Shoe {
    purpose = 'woodcutting'
}

class Sneaker implements Shoe {
    purpose = 'walking'
}

type CreateShoe = {
    create(type: 'balletFlat'): BalletFlat
    create(type: 'boot'): Shoe
    create(type: 'sneaker'): Shoe
}

let Shoe: CreateShoe = {
    create(type: 'balletFlat' | 'boot' | 'sneaker') {
        switch (type) {
            case 'balletFlat':
                return new BalletFlat
            case 'boot':
                return new Boot
            case 'sneaker':
                return new Sneaker
        }
    }
}

Shoe.create('balletFlat');

//[Hard] As an exercise, think about how you might design a typesafe builder pattern. Extend the Builder pattern “Builder Pattern” to:
// Guarantee at compile time that someone can’t call send before setting at least a URL and a method.
// Would it be easier to make this guarantee if you also force the user to call methods in a specific order?
// (Hint: what can you return instead of this?)

// [Harder] How would you change your design if you wanted to make this guarantee, but still let people call methods in any order?
// (Hint: what TypeScript feature can you use to make each method’s return type “add” to the this type after each method call?)

type Method = 'get' | 'post' | null;

class RequestBuilder {
    protected method: Method = null
    protected url: string | null = null
    protected data: object | null = null

    setMethod(method: 'get' | 'post'): RequestBuilderWithMethod {
        this.method = method;
        return new RequestBuilderWithMethod().setMethod(this.method).setData(this.data);
    }

    setData(data: object | null): this {
        this.data = data
        return this
    }
}

class RequestBuilderWithMethod extends RequestBuilder {
    setMethod(method: 'get' | 'post' | null): this {
        this.method = method;
        return this;
    }

    setURL(url: string): RequestBuilderWithMethodAndUrl {
        this.url = url;
        return new RequestBuilderWithMethodAndUrl().setMethod(this.method).setData(this.data).setURL(this.url)
    }
}


class RequestBuilderWithMethodAndUrl extends RequestBuilderWithMethod {
    setURL(url: string) {
        this.url = url;
        return this;
    }

    send(): this {
        console.log(this)
        return this
    }
}

new RequestBuilder()
    .setMethod('get')
    .setURL('/users')
    .setData({firstName: 'Anna'})
    .send()

// So we would need the calls to return the class itself in order, but only return the class on the condition that something
// else has been set
