import { cn } from '@/lib/utils';
import type { AllStats } from '@/types';

interface KeyboardHeatmapProps {
  stats: AllStats;
}

const KEYBOARD_LAYOUT = [
  [
    '`',
    '1',
    '2',
    '3',
    '4',
    '5',
    '6',
    '7',
    '8',
    '9',
    '0',
    '-',
    '=',
    'Backspace',
  ],
  ['Tab', 'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '[', ']', '\\'],
  ['CapsLock', 'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';', "'", 'Return'],
  ['ShiftLeft', 'z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/', 'ShiftRight'],
  [
    'ControlLeft',
    'MetaLeft',
    'Alt',
    'Space',
    'AltGr',
    'MetaRight',
    'ControlRight',
  ],
];

const KEY_DISPLAY_NAMES: Record<string, string> = {
  Backspace: '⌫',
  Tab: 'Tab',
  CapsLock: 'Caps',
  Return: '⏎',
  Enter: '⏎',
  ShiftLeft: '⇧',
  ShiftRight: '⇧',
  ControlLeft: 'Ctrl',
  ControlRight: 'Ctrl',
  MetaLeft: '⌘',
  MetaRight: '⌘',
  Alt: 'Alt',
  AltGr: 'Alt',
  Space: 'Space',
};

const KEY_WIDTHS: Record<string, string> = {
  Backspace: 'w-20',
  Tab: 'w-16',
  CapsLock: 'w-18',
  Return: 'w-20',
  ShiftLeft: 'w-24',
  ShiftRight: 'w-28',
  Space: 'w-64',
  ControlLeft: 'w-16',
  ControlRight: 'w-16',
  MetaLeft: 'w-14',
  MetaRight: 'w-14',
  Alt: 'w-14',
  AltGr: 'w-14',
};

function getHeatColor(intensity: number): string {
  // Cold (low) = blue; Hot (high) = red
  if (intensity === 0) return 'bg-secondary/50';

  if (intensity < 0.04) return 'bg-blue-950/80'; // Extremely cold
  if (intensity < 0.08) return 'bg-blue-900/80'; // Ultra cold
  if (intensity < 0.13) return 'bg-blue-800/80'; // Very cold
  if (intensity < 0.19) return 'bg-blue-700/80'; // Cold
  if (intensity < 0.26) return 'bg-blue-600/80'; // Semi-cold
  if (intensity < 0.34) return 'bg-cyan-700/80'; // Cool
  if (intensity < 0.42) return 'bg-cyan-500/80'; // Slightly cool
  if (intensity < 0.52) return 'bg-green-400/80'; // Transition - cool/neutral
  if (intensity < 0.62) return 'bg-yellow-400/80'; // Neutral/warm
  if (intensity < 0.7) return 'bg-yellow-500/90'; // Slightly warm
  if (intensity < 0.77) return 'bg-orange-400/90'; // Warm
  if (intensity < 0.84) return 'bg-orange-500/90'; // Very warm
  if (intensity < 0.9) return 'bg-red-400/90'; // Hot
  if (intensity < 0.96) return 'bg-red-500/90'; // Very hot
  return 'bg-red-700/95'; // Extreme hot
}

function getKeyCount(stats: AllStats, keyName: string): number {
  const freq = stats.key_frequency_map;

  if (freq[keyName] !== undefined) return freq[keyName];

  const upper = keyName.toUpperCase();
  const lower = keyName.toLowerCase();
  const capitalized =
    keyName.charAt(0).toUpperCase() + keyName.slice(1).toLowerCase();

  if (freq[upper] !== undefined) return freq[upper];
  if (freq[lower] !== undefined) return freq[lower];
  if (freq[capitalized] !== undefined) return freq[capitalized];

  if (keyName.length === 1) {
    const keyVariant = `Key${upper}`;
    if (freq[keyVariant] !== undefined) return freq[keyVariant];
  }

  return 0;
}

function getIntensity(stats: AllStats, keyName: string): number {
  const count = getKeyCount(stats, keyName);
  const maxCount = Math.max(...Object.values(stats.key_frequency_map), 1);
  return count / maxCount;
}

export function KeyboardHeatmap({ stats }: KeyboardHeatmapProps) {
  return (
    <div className="flex flex-col items-center gap-4 p-10 bg-secondary/20 rounded-lg">
      {KEYBOARD_LAYOUT.map((row, rowIndex) => (
        <div key={rowIndex} className="flex gap-1">
          {row.map((key) => {
            const intensity = getIntensity(stats, key);
            const count = getKeyCount(stats, key);
            const displayName = KEY_DISPLAY_NAMES[key] || key.toUpperCase();
            const width = KEY_WIDTHS[key] || 'w-12';

            return (
              <div
                key={key}
                className={cn(
                  'h-12 flex items-center justify-center rounded-md border border-border text-foreground text-xs font-medium transition-all hover:scale-105 relative group cursor-default',
                  width,
                  getHeatColor(intensity)
                )}
                title={`${key}: ${count} presses`}
              >
                <span className="truncate px-1">{displayName}</span>
                {count > 0 && (
                  <div className="absolute -top-8 left-1/2 -translate-x-1/2 bg-card text-foreground text-xs px-2 py-1 rounded opacity-0 group-hover:opacity-100 transition-opacity whitespace-nowrap z-10 border border-border">
                    {count.toLocaleString()} presses
                  </div>
                )}
              </div>
            );
          })}
        </div>
      ))}

      {/* Legend */}
      <div className="grid grid-cols-4 gap-3 mt-4 text-xs text-muted-foreground">
        <div className="flex items-center gap-1">
          <div className="w-6 h-4 bg-secondary/50 rounded border border-border" />
          <span>None</span>
        </div>
        <div className="flex items-center gap-1">
          <div className="w-6 h-4 bg-blue-950/80 rounded border border-border" />
          <span>Ultra Cold</span>
        </div>
        <div className="flex items-center gap-1">
          <div className="w-6 h-4 bg-blue-900/80 rounded border border-border" />
          <span>Ext. Cold</span>
        </div>
        <div className="flex items-center gap-1">
          <div className="w-6 h-4 bg-blue-800/80 rounded border border-border" />
          <span>Very Cold</span>
        </div>
        <div className="flex items-center gap-1">
          <div className="w-6 h-4 bg-blue-700/80 rounded border border-border" />
          <span>Cold</span>
        </div>
        <div className="flex items-center gap-1">
          <div className="w-6 h-4 bg-blue-600/80 rounded border border-border" />
          <span>Semi-Cold</span>
        </div>
        <div className="flex items-center gap-1">
          <div className="w-6 h-4 bg-cyan-700/80 rounded border border-border" />
          <span>Cool</span>
        </div>
        <div className="flex items-center gap-1">
          <div className="w-6 h-4 bg-cyan-500/80 rounded border border-border" />
          <span>Slightly Cool</span>
        </div>
        <div className="flex items-center gap-1">
          <div className="w-6 h-4 bg-green-400/80 rounded border border-border" />
          <span>Neutral</span>
        </div>
        <div className="flex items-center gap-1">
          <div className="w-6 h-4 bg-yellow-400/80 rounded border border-border" />
          <span>Warm</span>
        </div>
        <div className="flex items-center gap-1">
          <div className="w-6 h-4 bg-yellow-500/90 rounded border border-border" />
          <span>Slightly Warm</span>
        </div>
        <div className="flex items-center gap-1">
          <div className="w-6 h-4 bg-orange-400/90 rounded border border-border" />
          <span>Warmer</span>
        </div>
        <div className="flex items-center gap-1">
          <div className="w-6 h-4 bg-orange-500/90 rounded border border-border" />
          <span>Hot</span>
        </div>
        <div className="flex items-center gap-1">
          <div className="w-6 h-4 bg-red-400/90 rounded border border-border" />
          <span>Very Hot</span>
        </div>
        <div className="flex items-center gap-1">
          <div className="w-6 h-4 bg-red-500/90 rounded border border-border" />
          <span>Ext. Hot</span>
        </div>
        <div className="flex items-center gap-1">
          <div className="w-6 h-4 bg-red-700/95 rounded border border-border" />
          <span>Extreme</span>
        </div>
      </div>
    </div>
  );
}
