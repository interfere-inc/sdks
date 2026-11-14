/* oxlint-disable import/no-default-export, typescript/require-await */
import type { GetServerSideProps } from "next";

export const getServerSideProps = (async ({ query }) => {
  const { marker } = query;

  if (marker !== undefined) {
    throw new Error(String(marker));
  }

  return { props: { healthy: true } };
}) satisfies GetServerSideProps;

export default function ServerProps() {
  return <h1>Server props</h1>;
}
