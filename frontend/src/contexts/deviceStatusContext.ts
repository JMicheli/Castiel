import { createContext, useContext } from "react";
import type { DeviceStatus } from "@api/status";

export interface DeviceStatusContextType {
  status: DeviceStatus | null;
  loading: boolean;
  error: Error | null;
  refreshStatus: () => Promise<void>;
}

export const DeviceStatusContext =
  createContext<DeviceStatusContextType | null>(null);
DeviceStatusContext.displayName = "DeviceStatusContext";

export function useDeviceStatusContext(): DeviceStatusContextType {
  const context = useContext(DeviceStatusContext);
  if (context === null) {
    throw new Error(
      "useDeviceStatusContext must be used within a DeviceStatusProvider"
    );
  }
  return context;
}
