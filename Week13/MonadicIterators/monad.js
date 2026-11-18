

const boxing_fn = x => [x];

const unboxing_fn = x => x[0];

const data = 2;

const monad = boxing_fn(data)
    .map(x => x + 1)
    .map(x => x * 3)
    .map(x => x / 2);

const result = unboxing_fn(monad);
console.log(result);
