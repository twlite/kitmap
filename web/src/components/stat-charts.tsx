'use client';

import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import {
  BarChart,
  Bar,
  XAxis,
  YAxis,
  CartesianGrid,
  ResponsiveContainer,
  PieChart,
  Pie,
  Cell,
  LineChart,
  Line,
  AreaChart,
  Area,
} from 'recharts';
import {
  ChartContainer,
  ChartTooltip,
  ChartTooltipContent,
} from '@/components/ui/chart';
import type { AllStats } from '@/types';
import { formatKeyName } from '@/lib/format-key-name';

interface ChartsProps {
  stats: AllStats;
}

const CHART_COLORS = [
  'var(--chart-1)',
  'var(--chart-2)',
  'var(--chart-3)',
  'var(--chart-4)',
  'var(--chart-5)',
];

const formatValue = (value: unknown): string => {
  if (value === undefined || value === null) return '0';
  if (typeof value === 'number') return value.toLocaleString();
  if (typeof value === 'string') {
    const num = Number(value);
    return isNaN(num) ? value : num.toLocaleString();
  }
  return String(value);
};

export function TopKeysChart({ stats }: ChartsProps) {
  const data = stats.top_keys.slice(0, 10).map((key) => ({
    name: formatKeyName(key.key_name),
    count: key.count,
    percentage: key.percentage.toFixed(1),
  }));

  return (
    <Card>
      <CardHeader>
        <CardTitle className="text-lg">🔝 Top 10 Keys</CardTitle>
      </CardHeader>
      <CardContent>
        <ChartContainer
          config={{
            count: {
              label: 'Presses',
              color: CHART_COLORS[0],
            },
          }}
          className="h-[300px]"
        >
          <ResponsiveContainer width="100%" height="100%">
            <BarChart data={data} layout="vertical">
              <CartesianGrid strokeDasharray="3 3" />
              <XAxis type="number" />
              <YAxis type="category" dataKey="name" width={80} />
              <ChartTooltip
                content={({ active, payload }) => {
                  if (!active || !payload?.[0]) return null;
                  return (
                    <ChartTooltipContent
                      labelFormatter={(value) => {
                        return `${formatValue(value)} (${
                          payload?.[0]?.payload?.percentage || 0
                        }%)` as React.ReactNode;
                      }}
                      labelKey="Presses"
                    />
                  );
                }}
              />
              <Bar
                dataKey="count"
                fill={CHART_COLORS[0]}
                radius={[0, 4, 4, 0]}
              />
            </BarChart>
          </ResponsiveContainer>
        </ChartContainer>
      </CardContent>
    </Card>
  );
}

export function TopCombosChart({ stats }: ChartsProps) {
  const data = stats.top_combos.slice(0, 8).map((combo) => ({
    name:
      combo.combo.length > 15
        ? combo.combo.substring(0, 15) + '...'
        : combo.combo,
    count: combo.count,
    fullName: combo.combo,
  }));

  if (data.length === 0) {
    return (
      <Card>
        <CardHeader>
          <CardTitle className="text-lg">⌨️ Top Key Combos</CardTitle>
        </CardHeader>
        <CardContent className="flex items-center justify-center h-[300px] text-muted-foreground">
          No key combinations recorded yet
        </CardContent>
      </Card>
    );
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle className="text-lg">⌨️ Top Key Combos</CardTitle>
      </CardHeader>
      <CardContent>
        <ChartContainer
          config={{
            count: {
              label: 'Count',
              color: CHART_COLORS[1],
            },
          }}
          className="h-[300px]"
        >
          <ResponsiveContainer width="100%" height="100%">
            <BarChart data={data}>
              <CartesianGrid strokeDasharray="3 3" />
              <XAxis dataKey="name" angle={-45} textAnchor="end" height={80} />
              <YAxis />
              <ChartTooltip
                content={({ active, payload }) => {
                  if (!active || !payload?.[0]) return null;
                  const data = payload[0].payload as { fullName?: string };
                  return (
                    <ChartTooltipContent
                      labelFormatter={(value) => {
                        return `${formatValue(value)} (${
                          payload?.[0]?.payload?.percentage || 0
                        }%)` as React.ReactNode;
                      }}
                      labelKey={data?.fullName || 'Combo'}
                    />
                  );
                }}
              />
              <Bar
                dataKey="count"
                fill={CHART_COLORS[1]}
                radius={[4, 4, 0, 0]}
              />
            </BarChart>
          </ResponsiveContainer>
        </ChartContainer>
      </CardContent>
    </Card>
  );
}

export function HourlyActivityChart({ stats }: ChartsProps) {
  const data = stats.hourly_distribution.map((hour) => ({
    hour: `${hour.hour.toString().padStart(2, '0')}:00`,
    count: hour.count,
  }));

  return (
    <Card>
      <CardHeader>
        <CardTitle className="text-lg">⏰ Hourly Activity</CardTitle>
      </CardHeader>
      <CardContent>
        <ChartContainer
          config={{
            count: {
              label: 'Keys pressed',
              color: CHART_COLORS[2],
            },
          }}
          className="h-[300px]"
        >
          <ResponsiveContainer width="100%" height="100%">
            <AreaChart data={data}>
              <CartesianGrid strokeDasharray="3 3" />
              <XAxis dataKey="hour" interval={2} />
              <YAxis />
              <ChartTooltip content={<ChartTooltipContent />} />
              <defs>
                <linearGradient id="colorHourly" x1="0" y1="0" x2="0" y2="1">
                  <stop
                    offset="5%"
                    stopColor={CHART_COLORS[2]}
                    stopOpacity={0.8}
                  />
                  <stop
                    offset="95%"
                    stopColor={CHART_COLORS[2]}
                    stopOpacity={0}
                  />
                </linearGradient>
              </defs>
              <Area
                type="monotone"
                dataKey="count"
                stroke={CHART_COLORS[2]}
                fillOpacity={1}
                fill="url(#colorHourly)"
              />
            </AreaChart>
          </ResponsiveContainer>
        </ChartContainer>
      </CardContent>
    </Card>
  );
}

export function DailyActivityChart({ stats }: ChartsProps) {
  const data = stats.daily_distribution;

  return (
    <Card>
      <CardHeader>
        <CardTitle className="text-lg">📅 Daily Activity</CardTitle>
      </CardHeader>
      <CardContent>
        <ChartContainer
          config={{
            count: {
              label: 'Keys pressed',
              color: CHART_COLORS[3],
            },
          }}
          className="h-[300px]"
        >
          <ResponsiveContainer width="100%" height="100%">
            <LineChart data={data}>
              <CartesianGrid strokeDasharray="3 3" />
              <XAxis dataKey="day" />
              <YAxis />
              <ChartTooltip content={<ChartTooltipContent />} />
              <Line
                type="monotone"
                dataKey="count"
                stroke={CHART_COLORS[3]}
                strokeWidth={3}
                dot={{ fill: CHART_COLORS[3], strokeWidth: 2 }}
              />
            </LineChart>
          </ResponsiveContainer>
        </ChartContainer>
      </CardContent>
    </Card>
  );
}

export function KeyTypesPieChart({ stats }: ChartsProps) {
  const data = [
    { name: 'Letters', value: stats.letter_keys_count, color: CHART_COLORS[0] },
    { name: 'Numbers', value: stats.number_keys_count, color: CHART_COLORS[2] },
    {
      name: 'Modifiers',
      value: stats.modifier_keys_count,
      color: CHART_COLORS[3],
    },
    {
      name: 'Special',
      value: stats.special_keys_count,
      color: CHART_COLORS[1],
    },
  ].filter((item) => item.value > 0);

  return (
    <Card>
      <CardHeader>
        <CardTitle className="text-lg">📊 Key Types Distribution</CardTitle>
      </CardHeader>
      <CardContent>
        <ChartContainer config={{}} className="h-[300px]">
          <ResponsiveContainer width="100%" height="100%">
            <PieChart>
              <Pie
                data={data}
                cx="50%"
                cy="50%"
                innerRadius={60}
                outerRadius={100}
                paddingAngle={5}
                dataKey="value"
                label={({ name, percent }) =>
                  `${name} ${((percent ?? 0) * 100).toFixed(0)}%`
                }
                labelLine={false}
              >
                {data.map((entry, index) => (
                  <Cell key={`cell-${index}`} fill={entry.color} />
                ))}
              </Pie>
              <ChartTooltip content={<ChartTooltipContent />} />
            </PieChart>
          </ResponsiveContainer>
        </ChartContainer>
      </CardContent>
    </Card>
  );
}

export function SpecialKeysChart({ stats }: ChartsProps) {
  const data = [
    { name: 'Spacebar', count: stats.spacebar_count },
    { name: 'Enter', count: stats.enter_count },
    { name: 'Backspace', count: stats.backspace_count },
    { name: 'Tab', count: stats.tab_count },
    { name: 'Escape', count: stats.escape_count },
    { name: 'Delete', count: stats.delete_count },
    { name: 'Arrows', count: stats.arrow_keys_count },
  ].filter((item) => item.count > 0);

  return (
    <Card>
      <CardHeader>
        <CardTitle className="text-lg">🎯 Special Keys</CardTitle>
      </CardHeader>
      <CardContent>
        <ChartContainer config={{}} className="h-[300px]">
          <ResponsiveContainer width="100%" height="100%">
            <BarChart data={data}>
              <CartesianGrid strokeDasharray="3 3" />
              <XAxis dataKey="name" />
              <YAxis />
              <ChartTooltip content={<ChartTooltipContent />} />
              <Bar dataKey="count" radius={[4, 4, 0, 0]}>
                {data.map((_, index) => (
                  <Cell
                    key={`cell-${index}`}
                    fill={CHART_COLORS[index % CHART_COLORS.length]}
                  />
                ))}
              </Bar>
            </BarChart>
          </ResponsiveContainer>
        </ChartContainer>
      </CardContent>
    </Card>
  );
}
