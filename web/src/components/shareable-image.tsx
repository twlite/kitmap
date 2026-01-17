'use client';

import { useRef, useCallback, useState } from 'react';
import { toPng } from 'html-to-image';
import { Button } from '@/components/ui/button';
import { Card, CardContent } from '@/components/ui/card';
import { Download, Share2, Check, X } from 'lucide-react';
import type { AllStats } from '@/types';
import { formatKeyName } from '@/lib/format-key-name';

interface ShareableImageProps {
  stats: AllStats;
}

export function ShareableImage({ stats }: ShareableImageProps) {
  const cardRef = useRef<HTMLDivElement>(null);
  const [status, setStatus] = useState<{
    type: 'success' | 'error' | null;
    message: string;
  }>({ type: null, message: '' });

  const formatNumber = (num: number) => {
    if (num >= 1000000) return (num / 1000000).toFixed(1) + 'M';
    if (num >= 1000) return (num / 1000).toFixed(1) + 'K';
    return num.toLocaleString();
  };

  const showStatus = (type: 'success' | 'error', message: string) => {
    setStatus({ type, message });
    setTimeout(() => setStatus({ type: null, message: '' }), 3000);
  };

  const handleDownload = useCallback(async () => {
    if (!cardRef.current) return;

    try {
      const dataUrl = await toPng(cardRef.current, {
        cacheBust: true,
        backgroundColor: 'hsl(var(--color-background))',
        pixelRatio: 2,
      });

      const link = document.createElement('a');
      link.download = 'keyboard-stats.png';
      link.href = dataUrl;
      link.click();
      showStatus('success', 'Image downloaded!');
    } catch (error) {
      console.error('Failed to generate image:', error);
      showStatus('error', 'Failed to download image');
    }
  }, []);

  const handleCopyToClipboard = useCallback(async () => {
    if (!cardRef.current) return;

    try {
      const dataUrl = await toPng(cardRef.current, {
        cacheBust: true,
        backgroundColor: 'hsl(var(--color-background))',
        pixelRatio: 2,
      });

      const response = await fetch(dataUrl);
      const blob = await response.blob();

      await navigator.clipboard.write([
        new ClipboardItem({
          [blob.type]: blob,
        }),
      ]);

      showStatus('success', 'Image copied to clipboard!');
    } catch (error) {
      console.error('Failed to copy image:', error);
      showStatus('error', 'Failed to copy. Try downloading instead.');
    }
  }, []);

  const topKey = stats.most_pressed_key?.key_name || 'N/A';
  const topCombo = stats.most_pressed_combo?.combo || 'N/A';

  return (
    <Card>
      <CardContent className="p-6">
        <div className="flex items-center justify-between mb-6">
          <div className="flex items-center gap-3">
            <h3 className="text-lg font-semibold">📸 Share Your Stats</h3>
            {status.type && (
              <div
                className={`flex items-center gap-1 text-xs px-3 py-1 rounded-full ${
                  status.type === 'success'
                    ? 'bg-green-500/20 text-green-700 dark:text-green-400'
                    : 'bg-destructive/20 text-destructive'
                }`}
              >
                {status.type === 'success' ? (
                  <Check className="w-3 h-3" />
                ) : (
                  <X className="w-3 h-3" />
                )}
                {status.message}
              </div>
            )}
          </div>
          <div className="flex gap-2">
            <Button onClick={handleCopyToClipboard} variant="outline" size="sm">
              <Share2 className="w-4 h-4 mr-2" />
              Copy Image
            </Button>
            <Button onClick={handleDownload} size="sm">
              <Download className="w-4 h-4 mr-2" />
              Download
            </Button>
          </div>
        </div>

        {/* Shareable Card Preview */}
        <div className="flex justify-center">
          <div
            ref={cardRef}
            className="w-[600px] p-8 rounded-lg bg-card border border-border"
          >
            {/* Header */}
            <div className="text-center mb-6">
              <h1 className="text-3xl font-bold text-foreground mb-2">
                ⌨️ My Keyboard Stats
              </h1>
              <p className="text-muted-foreground text-sm">
                Generated with kitmap
              </p>
            </div>

            {/* Main Stats */}
            <div className="grid grid-cols-2 gap-4 mb-6">
              <div className="bg-secondary/30 rounded-lg p-4 text-center border border-border">
                <div className="text-4xl font-bold text-primary mb-1">
                  {formatNumber(stats.total_keys)}
                </div>
                <div className="text-muted-foreground text-sm">
                  Total Keys Pressed
                </div>
              </div>
              <div className="bg-secondary/30 rounded-lg p-4 text-center border border-border">
                <div className="text-4xl font-bold text-primary mb-1">
                  {formatNumber(stats.total_combos)}
                </div>
                <div className="text-muted-foreground text-sm">Key Combos</div>
              </div>
            </div>

            {/* Secondary Stats */}
            <div className="grid grid-cols-3 gap-3 mb-6">
              <div className="bg-secondary/20 rounded-md p-3 text-center border border-border">
                <div className="text-2xl font-bold text-primary">
                  {stats.keys_per_minute_avg.toFixed(0)}
                </div>
                <div className="text-muted-foreground text-xs">Keys/Min</div>
              </div>
              <div className="bg-secondary/20 rounded-md p-3 text-center border border-border">
                <div className="text-2xl font-bold text-primary">
                  {stats.unique_keys_used}
                </div>
                <div className="text-muted-foreground text-xs">Unique Keys</div>
              </div>
              <div className="bg-secondary/20 rounded-md p-3 text-center border border-border">
                <div className="text-2xl font-bold text-primary">
                  {stats.total_sessions}
                </div>
                <div className="text-muted-foreground text-xs">Sessions</div>
              </div>
            </div>

            {/* Favorites */}
            <div className="bg-secondary/20 rounded-lg p-4 mb-4 border border-border">
              <div className="grid grid-cols-2 gap-4 text-center">
                <div>
                  <div className="text-muted-foreground text-xs mb-1">
                    Most Pressed Key
                  </div>
                  <div className="text-xl font-bold text-primary">
                    {formatKeyName(topKey)}
                  </div>
                </div>
                <div>
                  <div className="text-muted-foreground text-xs mb-1">
                    Top Combo
                  </div>
                  <div className="text-xl font-bold text-primary truncate">
                    {formatKeyName(topCombo)}
                  </div>
                </div>
              </div>
            </div>

            {/* Special Keys Row */}
            <div className="flex justify-center gap-4 mb-6">
              <div className="text-center">
                <div className="text-lg font-bold text-foreground">
                  {formatNumber(stats.spacebar_count)}
                </div>
                <div className="text-muted-foreground text-xs">Spacebar</div>
              </div>
              <div className="text-center">
                <div className="text-lg font-bold text-foreground">
                  {formatNumber(stats.enter_count)}
                </div>
                <div className="text-muted-foreground text-xs">Enter</div>
              </div>
              <div className="text-center">
                <div className="text-lg font-bold text-foreground">
                  {formatNumber(stats.backspace_count)}
                </div>
                <div className="text-muted-foreground text-xs">Backspace</div>
              </div>
            </div>

            {/* Watermark */}
            <div className="text-center">
              <p className="text-muted-foreground text-xs">
                created with{' '}
                <a
                  href="https://github.com/twlite/kitmap"
                  className="text-primary font-semibold"
                >
                  kitmap
                </a>{' '}
                by{' '}
                <a
                  href="https://twlite.dev?utm_source=kitmap&utm_medium=image"
                  className="text-primary font-semibold"
                >
                  Twilight
                </a>
              </p>
            </div>
          </div>
        </div>
      </CardContent>
    </Card>
  );
}
