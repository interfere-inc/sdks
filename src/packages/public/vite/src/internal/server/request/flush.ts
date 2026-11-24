/** Node exports request spans on the batch timer, so a finished request schedules nothing. */
// oxlint-disable-next-line eslint/no-empty-function -- Node is the runtime with nothing to schedule; the workerd variant flushes.
export function flushAfterRequest() {}
