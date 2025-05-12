/**
 * TypeScript interfaces and API functions for server configuration.
 */

/**
 * A response from the server's version endpoint.
 */
export interface ServerVersionInfo {
  /** The server version from Cargo.toml */
  version: string;
}

/**
 * Fetches the current server version.
 *
 * @returns Promise resolving to a ServerVersionInfo object.
 */
export async function fetchVersion(): Promise<ServerVersionInfo> {
  try {
    const response = await fetch("/api/version");

    if (!response.ok) {
      throw new Error(
        `Failed to fetch server version: ${response.status} ${response.statusText}`
      );
    }

    return (await response.json()) as ServerVersionInfo;
  } catch (error) {
    console.error("Error fetching Chromecasts:", error);
    return { version: "Unknown" };
  }
}
