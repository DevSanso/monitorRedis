export const get_unix_epoch = (time : number) : number => {
    const nowInSeconds = Math.floor(time / 1000);
    return nowInSeconds;
}
