export function minutesSecondToMilliseconds(min: number, sec: number): number {
    return ((min*60) + sec) * 1000;
}

export function prettyFromMilliseconds(milli: number): string {
    const minutes = Math.floor(milli / 60000);
    const seconds = ((milli % 60000) / 1000).toFixed(0);
    return (Number(seconds) == 60 ? (minutes+1) + ":00" : minutes + ":" + (Number(seconds) < 10 ? "0" : "") + seconds);
}

export function millisecondsToMinutesSeconds(milli: number): [number, number] {
    return [
        Math.floor((milli / (1000 * 60))),
        Math.floor((milli / 1000) % 60)
    ];
}

export function millisecondsToHourMinutes(milli: number): [number, number] {
    return [
        Math.floor((milli / (1000 * 60 * 60))),
        Math.floor((milli / (1000 * 60)) % 60)
    ]
}

export function prettyHourMinuteFromMilliseconds(milli: number): string {
    const hm = millisecondsToHourMinutes(milli);
    const h = hm[0];
    const m = hm[1];
    return (Number(m) == 60 ? (h+1) + ":00" : h + ":" + (Number(m) < 10 ? "0" : "") + m);
}