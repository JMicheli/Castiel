/**
 * TypeScript interfaces and API functions for Chromecast media control.
 */

import type { DiscoveredDevice } from "./discovery";

export type ReceiverOptions = "Default" | "YouTube" | "Web";

export type StreamTypeOptions = "Live" | "Buffered" | "None";

/**
 * Sends media data to a specific Chromecast receiver.
 *
 * @param device - The Chromecast device to send media to.
 * @param mediaSettings - The media settings (receiver type, content ID, etc.).
 * @returns Promise resolving when the media data is sent.
 */
export async function sendMediaToReceiver(
  device: DiscoveredDevice,
  mediaSettings: {
    receiver: ReceiverOptions;
    mediaUrl: string;
    contentType: string;
    streamType: StreamTypeOptions;
  }
): Promise<void> {
  try {
    const response = await fetch("api/start-media", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        ip_address: device.ip_address,
        port: device.port,
        receiver: mediaSettings.receiver,
        media_url: mediaSettings.mediaUrl,
        content_type: mediaSettings.contentType,
        stream_type: mediaSettings.streamType,
      }),
    });

    if (!response.ok) {
      throw new Error(
        `Failed to send media to receiver: ${response.status} ${response.statusText}`
      );
    }

    console.log("Media data sent successfully!");
  } catch (error) {
    console.error("Error sending media data:", error);
    // Re-throw to allow calling component to handle
    throw error;
  }
}
