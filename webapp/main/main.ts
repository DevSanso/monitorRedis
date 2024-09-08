const test = async() => {
    console.log("test1");
    await monCore.utils.sleep(1);
    console.log("test5");
};


test();