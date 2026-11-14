/* oxlint-disable import/no-default-export */
export const dynamic = "force-dynamic";

export default async function RenderFailure({
  searchParams,
}: {
  searchParams: Promise<{ marker?: string }>;
}) {
  const { marker } = await searchParams;

  throw new Error(marker ?? "journey-rsc-render-error");
}
