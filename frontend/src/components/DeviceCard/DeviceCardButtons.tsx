import { SiChromecast } from "@icons-pack/react-simple-icons";
import { ArrowClockwise, Info, XSquare } from "@phosphor-icons/react";

interface DeviceCardButtonsProps {
  handleInfoClick: (e: React.MouseEvent) => void;
  handleRefresh: (e: React.MouseEvent) => void;
  handleStartMedia: (e: React.MouseEvent) => void;
  handleStop: (e: React.MouseEvent) => void;
  allowStop: boolean;
}

function DeviceCardButtons({
  handleInfoClick,
  handleRefresh,
  handleStartMedia,
  handleStop,
  allowStop,
}: DeviceCardButtonsProps) {
  const stopButtonClasses = allowStop ? "is-danger" : "is-danger is-light";
  const stopButtonText = allowStop
    ? "has-text-grey-dark"
    : "has-text-danger-light";

  return (
    <div className="columns is-mobile is-vcentered mt-3">
      {/* Three primary media controls */}
      <div className="column">
        <div className="buttons is-small is-grouped">
          <button
            className="button is-small"
            title="Show device info"
            onClick={handleInfoClick}
          >
            <span className="icon">
              <Info size={16} />
            </span>
          </button>
          <button
            className="button is-small"
            title="Refresh device"
            onClick={handleRefresh}
          >
            <span className="icon">
              <ArrowClockwise size={16} />
            </span>
          </button>
          <button
            className="button is-small is-info"
            title="Media settings"
            onClick={handleStartMedia}
          >
            <span className="icon">
              <SiChromecast size={16} />
            </span>
          </button>
        </div>
      </div>

      {/* Stop button is on the right side of the columns */}
      <div className="column is-narrow">
        <button
          className={`button is-small ${stopButtonClasses}`}
          title="Stop media"
          onClick={handleStop}
          disabled={!allowStop}
        >
          <span className="icon">
            <XSquare size={16} className={stopButtonText} />
          </span>
        </button>
      </div>
    </div>
  );
}

export default DeviceCardButtons;
