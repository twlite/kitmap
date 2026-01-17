import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import {
  Keyboard,
  MousePointerClick,
  Clock,
  Gauge,
  Hash,
  Calendar,
} from 'lucide-react';
import type { AllStats } from '@/types';

interface StatCardsProps {
  stats: AllStats;
}

export function StatCards({ stats }: StatCardsProps) {
  const formatNumber = (num: number) => {
    if (num >= 1000000) return (num / 1000000).toFixed(1) + 'M';
    if (num >= 1000) return (num / 1000).toFixed(1) + 'K';
    return num.toLocaleString();
  };

  const cards = [
    {
      title: 'Total Keys Pressed',
      value: formatNumber(stats.total_keys),
      icon: Keyboard,
    },
    {
      title: 'Key Combinations',
      value: formatNumber(stats.total_combos),
      icon: MousePointerClick,
    },
    {
      title: 'Recording Sessions',
      value: stats.total_sessions.toString(),
      icon: Clock,
    },
    {
      title: 'Avg. Speed (KPM)',
      value: stats.keys_per_minute_avg.toFixed(1),
      icon: Gauge,
    },
    {
      title: 'Unique Keys Used',
      value: stats.unique_keys_used.toString(),
      icon: Hash,
    },
    {
      title: 'Total Time (min)',
      value: stats.total_time_minutes.toFixed(0),
      icon: Calendar,
    },
  ];

  return (
    <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-4">
      {cards.map((card) => (
        <Card key={card.title}>
          <CardHeader className="flex flex-row items-center justify-between pb-2">
            <CardTitle className="text-sm font-medium text-muted-foreground">
              {card.title}
            </CardTitle>
            <div className="p-2 bg-primary/10 rounded-lg">
              <card.icon className="h-4 w-4 text-primary" />
            </div>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-foreground">
              {card.value}
            </div>
          </CardContent>
        </Card>
      ))}
    </div>
  );
}
