import { useState } from "react";
import type { DiscoveredDevice } from "@api/discovery";
import StartMediaModal from "./StartMediaModal";
import DeviceInfoModal from "./DeviceInfoModal";
import { useChromecastStatus } from "@hooks/useChromecastStatus";
import DeviceCardButtons from "./DeviceCardButtons";
import DeviceMediaInfo from "./DeviceMediaInfo";

interface ChromecastCardProps {
  device: DiscoveredDevice;
  onSelect?: (device: DiscoveredDevice) => void;
}

/**
 * A component for listing discovered chromecast devices for interaction.
 *
 * Displays all of the information about the device in a card layout.
 *
 * @param device - The device to display information for.
 */
function ChromecastCard({ device, onSelect }: ChromecastCardProps) {
  const [showInfoModal, setShowInfoModal] = useState(false); // State for the new info modal
  const [showStartMedia, setShowStartMedia] = useState(false);

  const { status, /*loading, error,*/ refreshStatus } = useChromecastStatus(
    device.ip_address,
    device.port
  );

  // Pull variables out of status
  const appStatus = status?.appStatus;
  const appIdentity = appStatus?.app_identity ?? "Unknown";

  const handleRefresh = (e: React.MouseEvent) => {
    e.stopPropagation();
    refreshStatus();
  };

  const handleStartMedia = (e: React.MouseEvent) => {
    e.stopPropagation();
    setShowStartMedia(true);
  };

  const handleInfoClick = (e: React.MouseEvent) => {
    e.stopPropagation();
    setShowInfoModal(true); // Open the info modal
  };

  const handleClick = () => {
    if (onSelect) {
      onSelect(device);
    }
  };

  return (
    <>
      <div className="card">
        <div className="card-header"></div>
        <div className="card-content" onClick={handleClick}>
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
            <DeviceMediaInfo appIdentity={appIdentity} />

            {/* Buttons for info / refresh / casting */}
            <DeviceCardButtons
              handleInfoClick={handleInfoClick}
              handleStartMedia={handleStartMedia}
              handleRefresh={handleRefresh}
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
    </>
  );
}

export default ChromecastCard;
