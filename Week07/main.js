function repeatFn(callbackFn) {
    callbackFn();
    callbackFn();
}
    
const fn = () => {
    console.log("Hello world!")
};

repeatFn(fn);

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


function map(list, fn){
    for(let i = 0; i < list.length; i++){
        const newValue = fn(list[i]);
        list[i] = newValue;
    }
    return list;
}

function addOne(value){
    return value + 1;
}

const arr = [1, 2, 3, 4, 5];
arr.forEach((value) => {
    console.log("value was " + value);
})

// const newList = map(arr, addOne);

// list.map((value) => value)
// console.log(newList);