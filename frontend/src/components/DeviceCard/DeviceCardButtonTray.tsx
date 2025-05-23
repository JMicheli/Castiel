import type { DiscoveredDevice } from "@api/discovery";
import { stopMediaAtReceiver } from "@api/media";
import { useDeviceStatusContext } from "@contexts/deviceStatusContext";
import { SiChromecast } from "@icons-pack/react-simple-icons";
import { ArrowClockwise, Info, XSquare } from "@phosphor-icons/react";

/** Props for the `DeviceCardButtonTray` component. */
interface DeviceCardButtonTrayProps {
  /** The callback function for when the info button is pressed. */
  handleInfoClick: (e: React.MouseEvent) => void;
  /** The callback function for when the media start button is pressed. */
  handleStartMedia: (e: React.MouseEvent) => void;
  /** The device which this button tray controls */
  device: DiscoveredDevice;
}

export default function DeviceCardButtonTray({
  handleInfoClick,
  handleStartMedia,
  device,
}: DeviceCardButtonTrayProps) {
  const { status, refreshStatus } = useDeviceStatusContext();

  const appIdentity = status?.app_status?.app_identity ?? "Unknown";
  const allowStop = appIdentity != "Backdrop";

  const stopButtonClasses = allowStop ? "is-danger" : "is-danger is-light";
  const stopButtonText = allowStop
    ? "has-text-grey-dark"
    : "has-text-danger-light";
  const stopButtonCursor = allowStop ? "pointer" : "default";

  const handleRefresh = (e: React.MouseEvent) => {
    e.stopPropagation();
    refreshStatus();
  };

  const handleStop = (e: React.MouseEvent) => {
    e.stopPropagation();
    // Stop the media playing
    stopMediaAtReceiver(device);
    // Refresh device status in 500ms.
    setTimeout(() => refreshStatus(), 500);
  };

  return (
    <div className="columns is-mobile is-vcentered mt-3">
      <div className="column">
        <div className="buttons is-small is-grouped">
          {/* Device info button */}
          <button
            className="button is-small"
            title="Show device info"
            aria-label="Show device info"
            onClick={handleInfoClick}
          >
            <span className="icon">
              <Info size={16} aria-hidden="true" />
            </span>
          </button>

          {/* Refresh device button */}
          <button
            className="button is-small"
            title="Refresh device"
            aria-label="Refresh device"
            onClick={handleRefresh}
          >
            <span className="icon">
              <ArrowClockwise size={16} aria-hidden="true" />
            </span>
          </button>

          {/* Media start button */}
          <button
            className="button is-small is-info"
            title="Start media"
            aria-label="Start media"
            onClick={handleStartMedia}
          >
            <span className="icon">
              <SiChromecast size={16} aria-hidden="true" />
            </span>
          </button>
        </div>
      </div>

      {/* Stop media button */}
      <div className="column is-narrow">
        <button
          className={`button is-small ${stopButtonClasses}`}
          title="Stop media"
          aria-label="Stop media"
          onClick={handleStop}
          disabled={!allowStop}
          style={{ cursor: stopButtonCursor }}
        >
          <span className="icon">
            <XSquare size={16} className={stopButtonText} aria-hidden="true" />
          </span>
        </button>
      </div>
    </div>
  );
}
