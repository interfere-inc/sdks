/** What a failed fetch settled with: the thrown value when it is an Error, otherwise its message. */
export type FetchError = Error | { message: string; status?: number };

export type FetchSpanRequest = Request | RequestInfo | RequestInit | URL;

export interface PageExitWatch {
  dispose: () => void;
  leftSince: (time: number) => boolean;
  initiatorSince?: (time: number) => "unload" | undefined;
}
