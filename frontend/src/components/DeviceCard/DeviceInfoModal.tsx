import type { DiscoveredDevice } from "@api/discovery";

interface DeviceInfoModalProps {
  device: DiscoveredDevice;
  isActive: boolean;
  onClose: () => void;
}

export default function DeviceInfoModal({
  device,
  isActive,
  onClose,
}: DeviceInfoModalProps) {
  const modalClass = `modal ${isActive ? "is-active" : ""}`;

  return (
    <div className={modalClass}>
      <div className="modal-background" onClick={onClose}></div>
      <div className="modal-card">
        <header className="modal-card-head">
          <p className="modal-card-title">Device Information</p>
          <button
            className="delete"
            aria-label="close"
            onClick={onClose}
          ></button>
        </header>
        <section className="modal-card-body">
          <div className="content">
            <p>
              <strong>Device ID:</strong> {device.id}
            </p>
            <p>
              <strong>Full Name:</strong> {device.fullname}
            </p>

            <h4>TXT Properties:</h4>
            <table className="table is-striped is-narrow is-hoverable is-fullwidth">
              <thead>
                <tr>
                  <th>Property</th>
                  <th>Value</th>
                </tr>
              </thead>
              <tbody>
                {Object.entries(device.txt_properties).map(([key, value]) => (
                  <tr key={key}>
                    <td>{key}</td>
                    <td>{value}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </section>
        <footer className="modal-card-foot">
          <button className="button" onClick={onClose}>
            Close
          </button>
        </footer>
      </div>
    </div>
  );
}
