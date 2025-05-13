import { useEffect, useState } from "react";
import { fetchVersion } from "@api/config";
import type { ServerVersionInfo } from "@api/config";

export default function About() {
  const [serverVersion, setServerVersion] = useState<ServerVersionInfo | null>(
    null
  );
  const [serverError, setServerError] = useState<string | null>(null);

  useEffect(() => {
    const getServerVersion = async () => {
      try {
        const versionInfo = await fetchVersion();
        setServerVersion(versionInfo);
      } catch (error) {
        setServerError("Failed to fetch server version.");
        console.error("Error fetching server version:", error);
      }
    };

    getServerVersion();
  }, []);

  // Frontend version from package.json - needs to be exposed via environment variable
  const frontendVersion = process.env.VITE_APP_VERSION || "N/A";

  return (
    <>
      <h1 className="title is-3">About Castiel</h1>

      {/* Description */}
      <div className="block">
        <p className="content">
          Castiel allows you to discover and control Chromecast devices on your
          network.
        </p>
      </div>

      {/* Server version */}
      <div className="block">
        <h2 className="subtitle is-5">Server Version</h2>
        {serverVersion ? (
          <span className="tag is-success is-medium">
            {serverVersion.version}
          </span>
        ) : serverError ? (
          <div className="notification is-danger is-light">{serverError}</div>
        ) : (
          <button className="button is-loading is-white is-inverted" disabled>
            Loading…
          </button>
        )}
      </div>

      {/* Frontend version */}
      <div className="block">
        <h2 className="subtitle is-5">Frontend Version</h2>
        <span className="tag is-info is-medium">{frontendVersion}</span>
      </div>
    </>
  );
}
