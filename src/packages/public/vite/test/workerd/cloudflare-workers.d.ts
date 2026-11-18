declare module "cloudflare:workers" {
  export abstract class WorkerEntrypoint {
    protected readonly env: unknown;
  }

  export const exports: {
    default: InstanceType<typeof import("./flush-worker").default>;
  };
}
