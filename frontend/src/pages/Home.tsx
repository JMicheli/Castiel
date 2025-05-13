import { useDeviceDiscovery } from "@hooks/useDeviceDiscovery";
import DeviceCard from "@components/DeviceCard/DeviceCard";

function Home() {
  const { devices, loading, error, refreshDevices } = useDeviceDiscovery(true);

  return (
    <>
      <div className="level">
        <div className="level-left">
          <div className="level-item">
            <h1 className="title">Chromecast Devices</h1>
          </div>
        </div>
        <div className="level-right">
          <div className="level-item">
            <button
              className={`button is-primary ${loading ? "is-loading" : ""}`}
              onClick={refreshDevices}
              disabled={loading}
            >
              Refresh Devices
            </button>
          </div>
        </div>
      </div>

      <div className="columns is-multiline mt-4">
        {devices.map((device) => (
          <div
            className="column is-one-third-desktop is-half-tablet is-full-mobile"
            key={device.fullname}
          >
            <DeviceCard device={device} />
          </div>
        ))}
      </div>

      {error && (
        <div className="notification is-danger">
          <button className="delete" onClick={() => {}}></button>
          Error: {error.message}
        </div>
      )}

      {loading && devices.length === 0 && (
        <div className="notification is-info">
          Searching for Chromecast devices...
        </div>
      )}

      {!loading && devices.length === 0 && (
        <div className="notification is-warning">
          No Chromecast devices found. Try refreshing the list.
        </div>
      )}
    </>
  );
}

export default Home;
