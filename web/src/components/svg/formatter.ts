export function nanoSeconds(value: number): string {
  if (value > 1000) {
    return microSeconds(value / 1000);
  } else {
    return (Math.round((value + Number.EPSILON) * 100) / 100).toString() + ' ns';
  }
}

export function microSeconds(value: number): string {
  if (value > 1000) {
    return milliSeconds(value / 1000);
  } else {
    return (Math.round((value + Number.EPSILON) * 100) / 100).toString() + ' µs';
  }
}

export function milliSeconds(value: number): string {
  if (value > 1000) {
    return seconds(value / 1000);
  } else {
    return (Math.round((value + Number.EPSILON) * 100) / 100).toString() + ' ms';
  }
}

export function seconds(value: number): string {
  if (value > 60) {
    return minutes(value / 60);
  } else {
    return (Math.round((value + Number.EPSILON) * 100) / 100).toString() + ' s';
  }
}

export function minutes(value: number): string {
  return (Math.round((value + Number.EPSILON) * 100) / 100).toString() + ' m';
}

