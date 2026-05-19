import { Maximize2, Minus, X } from 'lucide-react';
import { getAegisBridge } from '../services/agent';

export function WindowControls() {
  const controls = getAegisBridge()?.windowControls;

  return (
    <div className="no-drag flex items-center gap-2">
      <button className="window-button" title="Minimize" onClick={() => void controls?.minimize()}>
        <Minus className="h-4 w-4" />
      </button>
      <button className="window-button" title="Maximize" onClick={() => void controls?.maximize()}>
        <Maximize2 className="h-4 w-4" />
      </button>
      <button className="window-button close" title="Close" onClick={() => void controls?.close()}>
        <X className="h-4 w-4" />
      </button>
    </div>
  );
}
