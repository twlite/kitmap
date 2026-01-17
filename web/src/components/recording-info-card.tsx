import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import type { AllStats } from '@/types';

interface RecordingInfoCardProps {
  stats: AllStats;
}

export function RecordingInfoCard({ stats }: RecordingInfoCardProps) {
  return (
    <Card>
      <CardHeader>
        <CardTitle className="text-lg">📝 Recording Info</CardTitle>
      </CardHeader>
      <CardContent className="space-y-3">
        <div className="flex justify-between items-center p-3 bg-secondary/50 rounded-lg border border-border">
          <span className="text-muted-foreground">Avg. Typing Speed</span>
          <span className="font-semibold text-foreground">
            {stats.average_typing_speed.toFixed(1)} CPM
          </span>
        </div>
        <div className="flex justify-between items-center p-3 bg-secondary/50 rounded-lg border border-border">
          <span className="text-muted-foreground">Max Typing Speed</span>
          <span className="font-semibold text-foreground">
            {stats.max_typing_speed.toFixed(1)} CPM
          </span>
        </div>
        <div className="flex justify-between items-center p-3 bg-secondary/50 rounded-lg border border-border">
          <span className="text-muted-foreground">Avg. Keys/Session</span>
          <span className="font-semibold text-foreground">
            {stats.average_keys_per_session.toFixed(0)}
          </span>
        </div>
        {stats.first_recorded && (
          <div className="flex justify-between items-center p-3 bg-secondary/50 rounded-lg border border-border">
            <span className="text-muted-foreground">First Recorded</span>
            <span className="font-semibold text-foreground text-sm">
              {new Date(stats.first_recorded).toLocaleDateString()}
            </span>
          </div>
        )}
      </CardContent>
    </Card>
  );
}
