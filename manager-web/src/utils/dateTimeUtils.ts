export function formatDuration(milliseconds: number): string {
    const MILLISECOND = 1.0;
    const SECOND = 1000 * MILLISECOND;
    const MINUTE = 60 * SECOND;
    const HOUR = 60 * MINUTE;

    let resultString = ""

    let hours = Math.floor(milliseconds / HOUR);
    milliseconds -= hours * HOUR;
    let minutes = Math.floor(milliseconds / MINUTE);
    milliseconds -= minutes * MINUTE;
    let seconds = Math.floor(milliseconds / SECOND);
    milliseconds -= seconds * SECOND;

    if (hours > 0){
        resultString += hours + "h ";
    }
    if (minutes > 0 || resultString.length > 0){
        resultString += minutes + "m ";
    }
    if (seconds > 0 || resultString.length > 0){
        resultString += seconds + "s";
    }

    if (resultString.length == 0){
        resultString += milliseconds + "ms"
    }

    return resultString;
}
