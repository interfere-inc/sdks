export const LOCAL_DEV_RELEASE_BUDGET_MS = 5000;

export const RELEASE_CLIENT_RETRY = { maxRetries: 4, timeoutInSeconds: 10 };

// WARNING: the client sleeps for the gateway's 30-second Retry-After between retries, so a caller budget shorter than every attempt plus every wait aborts a call the retry policy would have completed.
export const RELEASE_REQUEST_BUDGET_MS =
  ((RELEASE_CLIENT_RETRY.maxRetries + 1) *
    RELEASE_CLIENT_RETRY.timeoutInSeconds +
    RELEASE_CLIENT_RETRY.maxRetries * 30 +
    10) *
  1000;
