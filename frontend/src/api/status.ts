export interface DeviceStatus {
  is_active_input: boolean;
  in_standby: boolean;
  volume: VolumeStatus;
  app_status?: AppStatus;
}

interface VolumeStatus {
  volume: number;
  muted: boolean;
}

interface AppStatus {
  id: string;
  app_identity: ParsedApp;
  display_name: string;
  namespaces: string[];
  session_id: string;
  status: string;
  transport_id: string;
}

export type ParsedApp =
  | "Backdrop"
  | "DefaultMedia"
  | "YouTube"
  | "WebView"
  | "Unknown";

/**
 * Fetches the status of a Chromecast device.
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

export interface MediaStatus {
  current_time?: number;
  playback_rate: number;
  player_state: PlayerState;
}

export type PlayerState = "Idle" | "Playing" | "Buffering" | "Paused";

/**
 * Fetches the status of media playing on a Chromecast device.
 *
 * @param ip The IP address of the device.
 * @param port The port of the device.
 * @returns A promise that resolves with the media status.
 */
export async function fetchMediaStatus(
  ip: string,
  port: number
): Promise<MediaStatus> {
  const response = await fetch("/api/media-status", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ ip, port }),
  });

  if (!response.ok) {
    throw new Error(`Failed to fetch media status: ${response.statusText}`);
  }

  const data = await response.json();
  return data as MediaStatus;
}
