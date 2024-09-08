const test = async() => {
    let logger = monCore.collection.NewEventLog<string>(10, localStorage);

    logger.push("test1");
    await monCore.utils.sleep(10);
    logger.push("test2");
};

test();