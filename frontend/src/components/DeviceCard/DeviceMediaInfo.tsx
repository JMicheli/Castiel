import { useDeviceStatusContext } from "@contexts/deviceStatusContext";
import {
  Question,
  YoutubeLogo,
  MonitorPlay,
  ImagesSquare,
  GlobeSimple,
} from "@phosphor-icons/react";
import type { JSX } from "react";

export default function DeviceMediaInfo() {
  const { status } = useDeviceStatusContext();

  const appIdentity = status?.app_status?.app_identity ?? "Unknown";

  // Config per appIdentity
  const configs: Record<
    string,
    {
      icon: JSX.Element;
      label: string;
      bgClass: string;
      textClass: string;
    }
  > = {
    Backdrop: {
      icon: <ImagesSquare size={48} className="has-text-white" />,
      label: "Backdrop",
      bgClass: "has-background-grey-lighter",
      textClass: "has-text-white",
    },
    DefaultMedia: {
      icon: <MonitorPlay size={48} className="has-text-info" />,
      label: "Default Media Receiver",
      bgClass: "has-background-info-light",
      textClass: "has-text-info",
    },
    YouTube: {
      icon: <YoutubeLogo size={48} weight="fill" className="has-text-white" />,
      label: "YouTube",
      bgClass: "has-background-danger",
      textClass: "has-text-white",
    },

    WebView: {
      icon: <GlobeSimple size={48} className="has-text-link" />,
      label: "Web Viewer",
      bgClass: "has-background-link-light",
      textClass: "has-text-link",
    },
    Unknown: {
      icon: <Question size={48} className="has-text-white" />,
      label: "Unknown",
      bgClass: "has-background-grey-lighter",
      textClass: "has-text-white",
    },
  };

  // Get variables according to chosen appIdentity
  const { icon, label, bgClass, textClass } =
    configs[appIdentity] ?? configs.Unknown;

  return (
    <div
      className={`box mt-4 is-flex is-justify-content-center is-align-items-center ${bgClass}`}
      style={{ height: "8rem" }}
    >
      <div className={`has-text-centered ${textClass}`}>
        {icon}
        <p className="mt-2 is-size-7 has-text-weight-bold">{label}</p>
      </div>
    </div>
  );
}
