function assert(condition: boolean, message?: string) {
    if (!condition) {
        throw new Error(message || 'Assertion failed');
    }
}

assert.equal = function (a: any, b: any, message?: string) {
    if (a !== b) {
        throw new Error(message || `Assertion failed: ${a} !== ${b}`);
    }
}

assert.notEqual = function (a: any, b: any) {
    if (a === b) {
        throw new Error(`Assertion failed: ${a} === ${b}`);
    }
}

assert.deepEqual = function (a: any, b: any) {
    if (JSON.stringify(a) !== JSON.stringify(b)) {
        throw new Error(`Assertion failed: ${JSON.stringify(a)} !== ${JSON.stringify(b)}`);
    }
}

assert.fail = function () {
    throw new Error('Assertion failed');
}

export default assert;
