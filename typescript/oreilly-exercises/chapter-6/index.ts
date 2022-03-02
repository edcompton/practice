// Write an Exclusive<T, U> type that computes the types that are in either T or U, but not both.
// For example, Exclusive<1 | 2 | 3, 2 | 3 | 4> should resolve to 1 | 4.
// Write out step by step how the typechecker evaluates Exclusive<1 | 2, 2 | 4>.

type Exclusive<T, U> = Exclude<T, U> | Exclude<U, T>

type Exclusive2 = Exclusive<1 | 2 | 3, 2 | 3 | 4>;


// Rewrite the example (from “Definite Assignment Assertions”) to avoid the definite assignment assertion.

const globalCache = {
    get(arg: string) {
        return arg
    }
}

let userId: string = fetchUser()

userId.toUpperCase()

function fetchUser() {
    return userId = globalCache.get('userId')
}
