/* oxlint-disable import/no-default-export */
import { InterfereProvider } from "@interfere/next/provider";
import { GlobalTrigger } from "./global-trigger";

export default function Layout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body>
        <InterfereProvider>
          <GlobalTrigger />
          {children}
        </InterfereProvider>
      </body>
    </html>
  );
}
