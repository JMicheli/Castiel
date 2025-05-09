import { useState } from "react";
import type { DiscoveredDevice } from "../api/discovery";

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
  const [showDetails, setShowDetails] = useState(false);

  const handleClick = () => {
    if (onSelect) {
      onSelect(device);
    }
  };

  return (
    <div className="card">
      <div className="card-content" onClick={handleClick}>
        <div className="content">
          {/* Tile / Subtitle*/}
          <div className="is-flex is-justify-content-space-between is-align-items-center mb-2">
            <h4 className="title is-4 mb-0">
              {device.friendly_name || "Unnamed Chromecast"}
            </h4>
          </div>
          <p className="subtitle is-6 mb-3">
            {device.model_name || "Unknown model"}
          </p>

          {/* Standard information */}
          <strong className="mb-1">IP Address</strong>
          <p>{`${device.ip_address}:${device.port}`}</p>

          <strong className="mb-1">Device ID</strong>
          <p style={{ wordBreak: "break-word" }}>{device.id || "Unknown"}</p>

          <strong className="mb-1">Device Fullname</strong>
          <p style={{ wordBreak: "break-word" }}>{device.fullname}</p>

          {/* Button to open properties table*/}
          <button
            className="button is-small is-info is-light mt-3"
            onClick={(e) => {
              e.stopPropagation();
              setShowDetails(!showDetails);
            }}
          >
            {showDetails ? "Hide Details" : "Show Details"}
          </button>
          {/* TXT properties table*/}
          {showDetails && (
            <div className="mt-4">
              <h6 className="title is-6">TXT Properties</h6>
              <div className="table-container">
                <table
                  className="table is-fullwidth is-striped is-narrow"
                  style={{ tableLayout: "fixed" }}
                >
                  <thead>
                    <tr>
                      <th>Key</th>
                      <th>Value</th>
                    </tr>
                  </thead>
                  <tbody>
                    {Object.entries(device.txt_properties).map(
                      ([key, value]) => (
                        <tr key={key}>
                          <td style={{ width: "30%" }}>{key}</td>
                          <td style={{ wordBreak: "break-word" }}>{value}</td>
                        </tr>
                      )
                    )}
                  </tbody>
                </table>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

export default ChromecastCard;
