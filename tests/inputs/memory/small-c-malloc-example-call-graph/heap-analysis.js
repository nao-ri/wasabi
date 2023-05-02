// Author: Michael Pradel

/*
 * Heap analysis (not sure yet what exactly it's supposed to do...)
 * TODO: work in progress
 */

(function () {

    let mallocLocation;
    let mallocSize
    let storeValueSum = 0
    let dropValueSum = 0
    let storenum = 0
    let dropnum = 0


    Wasabi.analysis = {
        start(location) {
            console.log(location, "start");
        },

        call_pre(location, targetFunc, args, indirectTableIdx) {
            // console.log("calltest", Wasabi.module.info.functions[targetFunc].export == "malloc", Wasabi.module.info.functions[targetFunc].export, location, targetFunc, args, indirectTableIdx)//デバック用
            if (Wasabi.module.info.functions[targetFunc].export == "malloc") {
                console.log("malloctest", location, targetFunc, args, indirectTableIdx)//デバック用
                mallocLocation = location;
                mallocSize = args[0];
            }
        },

        call_post(location, values) {
            if (mallocLocation !== undefined && mallocLocation.func == location.func && mallocLocation.instr == location.instr) {
                console.log(location, "malloc of size", mallocSize, "got address", values[0]);
                mallocLocation = undefined;
            }
        },

        load(location, op, memarg, value) {
            console.log(location, op, "value =", value, "from =", memarg);
        },

        store(location, op, memarg, value) {
            console.log("memory_store", location, op, "value =", value, "to =", memarg);
            storenum += 1

            if (Number.isInteger(value)) {
                storeValueSum += value
            }
            console.log("storenum=", storenum)
            console.log("storeValueSum=", storeValueSum)
            console.log("diff_store_print", "storeValueSum - dropValueSum", storeValueSum - dropValueSum)
        },

        memory_size(location, currentSizePages) {
            console.log(location, "memory_size, size (in pages) =", currentSizePages);

        },

        memory_grow(location, byPages, previousSizePages) {
            console.log(location, "memory_grow, delta (in pages) =", byPages, "previous size (in pages) =", previousSizePages);
        },
        drop(location, value) {
            console.log("memory_drop", "location", location, "value", value);
            dropnum += 1
            if (Number.isInteger(value)) {
                dropValueSum += value
            }
            console.log("dropnum=", dropnum)
            console.log("dropValueSum=", dropValueSum)
            console.log("diff_drop_print", "storeValueSum - dropValueSum", storeValueSum - dropValueSum)
        },

    };

})();
