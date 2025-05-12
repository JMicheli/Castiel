import { SiChromecast } from "@icons-pack/react-simple-icons";
import { ArrowClockwise, Info } from "@phosphor-icons/react";

interface DeviceCardButtonsProps {
  handleInfoClick: (e: React.MouseEvent) => void;
  handleRefresh: (e: React.MouseEvent) => void;
  handleStartMedia: (e: React.MouseEvent) => void;
}

function DeviceCardButtons({
  handleInfoClick,
  handleRefresh,
  handleStartMedia,
}: DeviceCardButtonsProps) {
  return (
    <div className="columns is-mobile is-vcentered mt-3">
      <div className="column is-narrow is-offset-auto">
        <div className="buttons is-small is-grouped is-right">
          {/* Info button */}
          <button
            className="button is-small"
            title="Show device info"
            onClick={handleInfoClick} // Use the new handler
          >
            <span className="icon">
              <Info size={16} />
            </span>
          </button>
          {/* Refresh button */}
          <button
            className="button is-small"
            title="Refresh device"
            onClick={handleRefresh}
          >
            <span className="icon">
              <ArrowClockwise size={16} />
            </span>
          </button>
          {/* Cast button */}
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
    </div>
  );
}

export default DeviceCardButtons;
