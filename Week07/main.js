function repeatFn(callbackFn) {
    callbackFn();
    callbackFn();
}
    
const fn = () => {
    console.log("Hello world!")
};

function createMultiplier(factor) {
    // factor=2
    return function(number) {
        return number * factor;
    }
}

const double = createMultiplier(2);
const triple = createMultiplier(3);

console.log(double(5));
console.log(triple(5));

// repeatFn(fn);