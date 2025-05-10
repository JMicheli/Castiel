interface VolumeStatus {
  volume: number;
  muted: boolean;
}

interface AppStatus {
  id: string;
  diplayName: string;
  namespaces: string[];
  sessionId: string;
  status: string;
  transportId: string;
}

export interface DeviceStatus {
  isActiveInput: boolean;
  inStandby: boolean;
  volume: VolumeStatus;
  appStatus: AppStatus[];
}

/**
 * Fetches the status of a specific Chromecast device.
 *
 * @param ip The IP address of the device.
 * @param port The port of the device.
 * @returns A promise that resolves with the device status.
 */
export async function fetchDeviceStatus(
  ip: string,
  port: number
): Promise<DeviceStatus> {
  const response = await fetch("/api/device-status", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ ip, port }),
  });

  if (!response.ok) {
    throw new Error(`Failed to fetch device status: ${response.statusText}`);
  }

  const data = await response.json();
  return data as DeviceStatus;
}
