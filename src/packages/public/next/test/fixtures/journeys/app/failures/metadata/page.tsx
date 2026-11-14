/* oxlint-disable import/no-default-export */
export const dynamic = "force-dynamic";

export async function generateMetadata({
  searchParams,
}: {
  searchParams: Promise<{ marker?: string }>;
}) {
  const { marker } = await searchParams;

  throw new Error(marker ?? "journey-metadata-error");
}

export default function MetadataFailure() {
  return <h1>Metadata page</h1>;
}
