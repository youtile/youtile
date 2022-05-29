export function secondsToDuration(seconds: number): String {
  const iso = new Date(seconds * 1000);
  const isostr = iso.toISOString();

  if (isostr.substring(11, 13) != "00") {
    return isostr.substring(11, 19);
  } else {
    return isostr.substring(14, 19);
  }
}