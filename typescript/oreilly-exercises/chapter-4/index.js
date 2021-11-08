"use strict";
// Challenge - Second arg must always be string
function call(f, ...args) {
    return f(...args);
}
function fill(length, value) {
    return Array.from({ length }, () => value);
}
call(fill, 10, 'a'); // evaluates to an array of 10 'a's
// Create a typesafe library
function is(firstArg, secondArg, ...args) {
    return firstArg === secondArg;
}
// Compare a string and a string
console.log(is('string', 'otherstrig')); // false
// Compare a boolean and a boolean
is(true, false); // false
// Compare a number and a number
is(42, 42); // true
// Comparing two different types should give a compile-time error
is(10, 'foo'); // Error TS2345: Argument of type '"foo"' is not assignable
// to parameter of type 'number'.
// [Hard] I should be able to pass any number of arguments
is([1], [1, 2], [1, 2, 3]); // false
