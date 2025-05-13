import { useState, useEffect, useCallback } from "react";
import { type DiscoveredDevice, fetchChromecasts } from "@api/discovery";

interface UseDeviceDiscoveryResult {
  /** List of discovered Chromecast devices */
  devices: DiscoveredDevice[];

  /** Whether devices are currently being fetched */
  loading: boolean;

  /** Any error that occurred during fetching */
  error: Error | null;

  /** Function to manually refresh the list of devices */
  refreshDevices: () => Promise<void>;
}

/**
 * Hook for managing Chromecast device discovery
 *
 * @param autoRefresh Whether to automatically refresh the device list on mount
 * @returns Object containing devices, loading state, error state, and refresh function
 */
export function useDeviceDiscovery(
  autoRefresh = true
): UseDeviceDiscoveryResult {
  const [devices, setDevices] = useState<DiscoveredDevice[]>([]);
  const [loading, setLoading] = useState<boolean>(false);
  const [error, setError] = useState<Error | null>(null);

  const refreshDevices = useCallback(async () => {
    setLoading(true);
    setError(null);

    try {
      const discoveredDevices = await fetchChromecasts();
      setDevices(discoveredDevices);
    } catch (err) {
      setError(
        err instanceof Error ? err : new Error("Unknown error occurred")
      );
    } finally {
      setLoading(false);
    }
  }, []);

  // Refresh devices on mount if autoRefresh is true
  useEffect(() => {
    if (autoRefresh) {
      refreshDevices();
    }
  }, [autoRefresh, refreshDevices]);

  return {
    devices,
    loading,
    error,
    refreshDevices,
  };
}
