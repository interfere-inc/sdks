/**
 * Global a build stamps into the page with the default OTel `service.name`
 * (the build integrations derive it from the package name). The kernel reads it
 * when `init()` is called without `serviceName`; an explicit value wins.
 */
export const SERVICE_NAME_GLOBAL = "__INTERFERE_SERVICE_NAME__";
