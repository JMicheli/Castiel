import { useState } from "react";
import type { DiscoveredDevice } from "@api/discovery";
import {
  sendMediaToReceiver,
  type ReceiverOptions,
  type StreamTypeOptions,
} from "@api/media";
import { useDeviceStatusContext } from "@contexts/deviceStatusContext";

interface StartMediaProps {
  device: DiscoveredDevice;
  onClose: () => void;
  isOpen: boolean;
}

/**
 * Modal component for starting new Chromecast media.
 *
 * @param device - The Chromecast device to control.
 * @param onClose - Function to call when closing the modal.
 * @param isOpen - Whether the modal is currently open.
 */
export default function StartMediaModal({
  device,
  onClose,
  isOpen,
}: StartMediaProps) {
  const { refreshStatus } = useDeviceStatusContext();

  const [receiverType, setReceiverType] = useState<ReceiverOptions>("Default");
  const [contentId, setContentId] = useState("");
  const [contentType, setContentType] = useState("");
  const [streamType, setStreamType] = useState<StreamTypeOptions>("None");

  const handleSendMedia = async () => {
    try {
      // Send media the media to be started
      await sendMediaToReceiver(device, {
        receiver: receiverType,
        mediaUrl: contentId,
        contentType,
        streamType,
      });
      // Refresh device status in 500ms.
      setTimeout(() => refreshStatus(), 500);
      onClose();
    } catch (error) {
      console.error("Failed to send media:", error);
    }
  };

  if (!isOpen) return null;

  return (
    <div className="modal is-active">
      <div className="modal-background" onClick={onClose}></div>
      <div className="modal-card">
        <header className="modal-card-head py-2">
          <p className="modal-card-title">
            {device.friendly_name || "Unnamed Chromecast"}
          </p>
          <button
            className="delete is-large"
            aria-label="close"
            onClick={onClose}
          ></button>
        </header>
        <section className="modal-card-body">
          <div className="content">
            <div className="is-flex is-justify-content-space-between is-size-7 mb-2">
              <strong>Device Model:</strong>
              <p className="ml-2">{device.model_name || "Unknown model"}</p>
            </div>

            <div className="is-flex is-justify-content-space-between is-size-7 mb-2">
              <strong>Device ID:</strong>
              <p className="ml-2" style={{ wordBreak: "break-word" }}>
                {device.id || "Unknown"}
              </p>
            </div>

            <div className="box">
              <div className="field">
                <label className="label">Receiver Type</label>
                <div className="control">
                  <div className="select">
                    <select
                      value={receiverType}
                      onChange={(e) =>
                        setReceiverType(e.target.value as ReceiverOptions)
                      }
                    >
                      <option value="Default">Default Media Receiver</option>
                      <option value="YouTube">YouTube Receiver</option>
                      <option value="Web">Web Receiver</option>
                    </select>
                  </div>
                </div>
              </div>

              <div className="field">
                <label className="label">Content ID (URL)</label>
                <div className="control">
                  <input
                    className="input"
                    type="text"
                    placeholder="e.g. http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4"
                    value={contentId}
                    onChange={(e) => setContentId(e.target.value)}
                  />
                </div>
              </div>
              <div className="field">
                <label className="label">Content Type (MIME)</label>
                <div className="control">
                  <input
                    className="input"
                    type="text"
                    placeholder="e.g. videos/mp4"
                    value={contentType}
                    onChange={(e) => setContentType(e.target.value)}
                  />
                </div>
              </div>
              <div className="field">
                <label className="label">Stream Type</label>
                <div className="control">
                  <div className="select">
                    <select
                      value={streamType}
                      onChange={(e) =>
                        setStreamType(e.target.value as StreamTypeOptions)
                      }
                    >
                      <option value="Buffered">Buffered</option>
                      <option value="Live">Live</option>
                      <option value="None">None</option>
                    </select>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </section>
        <footer className="modal-card-foot py-2">
          <button className="button" onClick={onClose}>
            Close
          </button>
          <button className="button is-primary" onClick={handleSendMedia}>
            Send to Receiver
          </button>
        </footer>
      </div>
    </div>
  );
}
