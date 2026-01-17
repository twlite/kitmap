import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { formatKeyName } from '@/lib/format-key-name';
import type { AllStats } from '@/types';

interface HighlightsCardProps {
  stats: AllStats;
}

export function HighlightsCard({ stats }: HighlightsCardProps) {
  return (
    <Card>
      <CardHeader>
        <CardTitle className="text-lg">🏆 Highlights</CardTitle>
      </CardHeader>
      <CardContent className="space-y-3">
        {stats.most_pressed_key && (
          <div className="flex justify-between items-center p-3 bg-secondary/50 rounded-lg border border-border">
            <span className="text-muted-foreground">Most Pressed Key</span>
            <span className="font-semibold text-foreground">
              {formatKeyName(stats.most_pressed_key.key_name)} (
              {stats.most_pressed_key.count.toLocaleString()}x)
            </span>
          </div>
        )}
        {stats.most_pressed_combo && (
          <div className="flex justify-between items-center p-3 bg-secondary/50 rounded-lg border border-border">
            <span className="text-muted-foreground">Top Combo</span>
            <span className="font-semibold text-foreground">
              {formatKeyName(stats.most_pressed_combo.combo)} (
              {stats.most_pressed_combo.count.toLocaleString()}x)
            </span>
          </div>
        )}
        {stats.most_active_hour && (
          <div className="flex justify-between items-center p-3 bg-secondary/50 rounded-lg border border-border">
            <span className="text-muted-foreground">Most Active Hour</span>
            <span className="font-semibold text-foreground">
              {stats.most_active_hour.hour.toString().padStart(2, '0')}:00
            </span>
          </div>
        )}
        {stats.most_active_day && (
          <div className="flex justify-between items-center p-3 bg-secondary/50 rounded-lg border border-border">
            <span className="text-muted-foreground">Most Active Day</span>
            <span className="font-semibold text-foreground">
              {stats.most_active_day.day}
            </span>
          </div>
        )}
      </CardContent>
    </Card>
  );
}
