export function getInitials(name: string | null, identifier: string) {
  if (name) {
    return name
      .split(" ")
      .map((part) => part[0])
      .join("")
      .toUpperCase()
      .slice(0, 2);
  }

  return identifier.slice(0, 2).toUpperCase();
}
