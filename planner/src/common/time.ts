export function minutesSecondToMinutes(min: number, sec: number): number {
    return ((min*60) + sec) * 1000;
}

export function prettyFromMilliseconds(milli: number): string {
    const minutes = Math.floor(milli / 60000);
    const seconds = ((milli % 60000) / 1000).toFixed(0);
    return (Number(seconds) == 60 ? (minutes+1) + ":00" : minutes + ":" + (Number(seconds) < 10 ? "0" : "") + seconds);
}

export function millisecondsToMinutesSeconds(milli: number): [number, number] {
    return [
        Math.floor((milli / (1000 * 60)) % 60),
        Math.floor((milli / 1000) % 60)
    ];
}