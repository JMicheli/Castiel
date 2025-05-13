import { useState } from "react";
import type { DiscoveredDevice } from "@api/discovery";
import StartMediaModal from "./StartMediaModal";
import DeviceInfoModal from "./DeviceInfoModal";
import DeviceCardButtonTray from "./DeviceCardButtonTray";
import DeviceMediaInfo from "./DeviceMediaInfo";
import { DeviceStatusProvider } from "@providers/DeviceStatusProvider";

interface DeviceCardProps {
  device: DiscoveredDevice;
}

/**
 * A component for listing discovered chromecast devices for interaction.
 *
 * Displays all of the information about the device in a card layout.
 *
 * @param device - The device to display information for.
 */
export default function DeviceCard({ device }: DeviceCardProps) {
  const [showInfoModal, setShowInfoModal] = useState(false);
  const [showStartMedia, setShowStartMedia] = useState(false);

  const handleStartMedia = (e: React.MouseEvent) => {
    e.stopPropagation();
    setShowStartMedia(true);
  };

  const handleInfoClick = (e: React.MouseEvent) => {
    e.stopPropagation();
    setShowInfoModal(true);
  };

  return (
    <DeviceStatusProvider ip={device.ip_address} port={device.port}>
      <div className="card">
        <div className="card-header"></div>
        <div className="card-content">
          <div className="content">
            {/* Title and Subtitle */}
            <div className="columns is-mobile is-vcentered">
              <div className="column">
                <h4 className="title is-4 mb-1">
                  {device.friendly_name || "Unnamed Chromecast"}
                </h4>
                <p className="subtitle is-6 mt-1 mb-3">
                  {device.model_name || "Unknown model"}
                </p>
              </div>
            </div>

            {/* The device's IP address */}
            <strong className="mb-1">IP Address</strong>
            <p
              style={{ wordBreak: "break-word" }}
            >{`${device.ip_address}:${device.port}`}</p>

            {/* Display of currently playing media and player controls */}
            <DeviceMediaInfo />

            {/* Buttons for info / refresh / casting / stopping media */}
            <DeviceCardButtonTray
              handleInfoClick={handleInfoClick}
              handleStartMedia={handleStartMedia}
              device={device}
            />
          </div>
        </div>
      </div>

      {/* Media settings modal */}
      <StartMediaModal
        device={device}
        isOpen={showStartMedia}
        onClose={() => setShowStartMedia(false)}
      />

      {/* Device info modal */}
      <DeviceInfoModal
        device={device}
        isActive={showInfoModal}
        onClose={() => setShowInfoModal(false)}
      />
    </DeviceStatusProvider>
  );
}
