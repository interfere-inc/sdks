/* oxlint-disable import/no-default-export */
import { InterfereProvider } from "@interfere/next/provider";

export default function Layout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body>
        <InterfereProvider consent>
          {children}
          <button type="button">Rage target</button>
        </InterfereProvider>
      </body>
    </html>
  );
}
