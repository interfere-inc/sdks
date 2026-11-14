export interface PageEventsConfig {
  /**
   * `false` stops click and submit capture while navigation, document-load
   * and resource timing continue. Defaults to `true`.
   */
  interactions?: boolean;
}

/** `true` enables page events, `false` disables them, and an object enables them with that config. */
export type PageEventsOption = boolean | PageEventsConfig;
