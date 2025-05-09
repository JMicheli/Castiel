/**
 * TypeScript interfaces and API functions for Chromecast discovery.
 */

/**
 * Represents a discovered Chromecast device sent by the server.
 */
export interface DiscoveredDevice {
  /** IPv4 or IPv6 address of the discovered Chromecast device */
  ip_address: string;

  /** Port on which the Chromecast service is listening (typically 8009) */
  port: number;

  /** Full mDNS service name, e.g. "Chromecast-<id>._googlecast._tcp.local." */
  fullname: string;

  /** Chromecast device ID (TXT key "id") */
  id?: string;

  /** Model name (TXT key "md") */
  model_name?: string;

  /** Friendly name (TXT key "fn") */
  friendly_name?: string;

  /** All raw TXT properties */
  txt_properties: Record<string, string>;
}

/**
 * Fetches the list of discovered Chromecast devices from the API.
 *
 * @returns Promise resolving to an array of DiscoveredDevice objects
 */
export async function fetchChromecasts(): Promise<DiscoveredDevice[]> {
  try {
    const response = await fetch("/api/chromecasts");

    if (!response.ok) {
      throw new Error(
        `Failed to fetch Chromecasts: ${response.status} ${response.statusText}`
      );
    }

    return (await response.json()) as DiscoveredDevice[];
  } catch (error) {
    console.error("Error fetching Chromecasts:", error);
    return [];
  }
}
