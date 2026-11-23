function wait(ms) {
    // return a blocking promise
    return new Promise(resolve => setTimeout(resolve, ms));
}
    
async function makeCoffee() {
    console.log("Starting coffee machine");
    
    // block for two seconds
    // don't block main, bc we are already 
    // in an async context
    await wait(2000);
    
    console.log("Coffee now is ready");
}
    
// execute in background
makeCoffee();
console.log("Checking phone while waiting for coffee");