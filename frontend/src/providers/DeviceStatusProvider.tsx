import {
  useState,
  useCallback,
  useEffect,
  useMemo,
  type ReactNode,
} from "react";
import { fetchDeviceStatus, type DeviceStatus } from "@api/status";
import { DeviceStatusContext } from "@contexts/deviceStatusContext";

interface DeviceStatusProviderProps {
  children: ReactNode;
  ip: string;
  port: number;
}

export function DeviceStatusProvider({
  children,
  ip,
  port,
}: DeviceStatusProviderProps) {
  const [status, setStatus] = useState<DeviceStatus | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<Error | null>(null);

  const fetchStatus = useCallback(async () => {
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
  }, [ip, port]);

  useEffect(() => {
    fetchStatus();
  }, [fetchStatus]);

  const value = useMemo(
    () => ({ status, loading, error, refreshStatus: fetchStatus }),
    [status, loading, error, fetchStatus]
  );

  return (
    <DeviceStatusContext.Provider value={value}>
      {children}
    </DeviceStatusContext.Provider>
  );
}
