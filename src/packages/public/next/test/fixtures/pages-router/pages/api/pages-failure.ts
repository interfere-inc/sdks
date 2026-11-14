/* oxlint-disable import/no-default-export */
import type { NextApiRequest, NextApiResponse } from "next";

export default function handler(
  request: NextApiRequest,
  response: NextApiResponse,
) {
  const { marker } = request.query;

  if (marker !== undefined) {
    throw new Error(String(marker));
  }

  response.status(200).json({ healthy: true });
}
