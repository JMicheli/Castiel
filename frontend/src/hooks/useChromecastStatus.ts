import { useState, useEffect, useCallback } from "react";
import { fetchDeviceStatus, type DeviceStatus } from "@api/status";

interface UseChromecastStatusResult {
  /** The status of the Chromecast device */
  status: DeviceStatus | null;

  /** Whether the status is currently being fetched */
  loading: boolean;

  /** Any error that occurred during fetching */
  error: Error | null;

  /** Function to manually refresh the device status */
  refreshStatus: () => Promise<void>;
}

/**
 * Hook for managing Chromecast device status
 *
 * @param ip The IP address of the device to fetch status for
 * @param port The port of the device to fetch status for
 * @returns Object containing device status, loading state, error state, and refresh function
 */
export function useChromecastStatus(
  ip: string,
  port: number
): UseChromecastStatusResult {
  const [status, setStatus] = useState<DeviceStatus | null>(null);
  const [loading, setLoading] = useState<boolean>(false);
  const [error, setError] = useState<Error | null>(null);

  const deviceStatusCallback = useCallback(async (ip: string, port: number) => {
    setLoading(true);
    setError(null);

    try {
      const deviceStatus = await fetchDeviceStatus(ip, port);
      setStatus(deviceStatus);
    } catch (err) {
      setError(
        err instanceof Error ? err : new Error("Unknown error occurred")
      );
    } finally {
      setLoading(false);
    }
  }, []);

  const refreshStatus = useCallback(async () => {
    deviceStatusCallback(ip, port);
  }, [ip, port, deviceStatusCallback]);

  // Fetch status on mount
  useEffect(() => {
    if (ip && port) {
      deviceStatusCallback(ip, port);
    }
  }, [ip, port, deviceStatusCallback]);

  return {
    status,
    loading,
    error,
    refreshStatus,
  };
}
