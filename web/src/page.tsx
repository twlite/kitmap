import { useEffect, useState } from 'react';
import { KeyboardHeatmap } from '@/components/keyboard-heatmap';
import { StatCards } from '@/components/stat-cards';
import {
  TopKeysChart,
  TopCombosChart,
  HourlyActivityChart,
  DailyActivityChart,
  KeyTypesPieChart,
  SpecialKeysChart,
} from '@/components/stat-charts';
import { ShareableImage } from '@/components/shareable-image';
import { HighlightsCard } from '@/components/highlights-card';
import { RecordingInfoCard } from '@/components/recording-info-card';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Keyboard, RefreshCw } from 'lucide-react';
import { Button } from '@/components/ui/button';
import type { AllStats } from '@/types';

function App() {
  const [stats, setStats] = useState<AllStats | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchStats = async () => {
    try {
      setLoading(true);
      setError(null);
      const response = await fetch('http://localhost:3456/api/stats');
      if (!response.ok) {
        throw new Error('Failed to fetch stats');
      }
      const data = await response.json();
      setStats(data);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'An error occurred');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchStats();
  }, []);

  if (loading) {
    return (
      <div className="min-h-screen bg-background flex items-center justify-center">
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-2 border-primary border-t-transparent mx-auto mb-4"></div>
          <p className="text-muted-foreground">Loading your kitmap stats...</p>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="min-h-screen bg-background flex items-center justify-center">
        <Card className="max-w-md">
          <CardContent className="p-8 text-center">
            <div className="text-destructive text-6xl mb-4">⚠️</div>
            <h2 className="text-xl font-bold text-foreground mb-2">
              Error Loading Stats
            </h2>
            <p className="text-muted-foreground mb-4">{error}</p>
            <Button onClick={fetchStats}>
              <RefreshCw className="w-4 h-4 mr-2" />
              Retry
            </Button>
          </CardContent>
        </Card>
      </div>
    );
  }

  if (!stats) {
    return null;
  }

  return (
    <div className="min-h-screen bg-background">
      {/* Header */}
      <header className="border-b border-border bg-card sticky top-0 z-50">
        <div className="container mx-auto px-4 py-6">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-3">
              <div className="p-2 bg-primary/10 rounded-lg">
                <Keyboard className="h-6 w-6 text-primary" />
              </div>
              <div>
                <h1 className="text-2xl font-bold text-foreground">kitmap</h1>
                <p className="text-xs text-muted-foreground">
                  Keyboard Activity Tracker
                </p>
              </div>
            </div>
            <Button onClick={fetchStats} variant="outline" size="sm">
              <RefreshCw className="w-4 h-4 mr-2" />
              Refresh
            </Button>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="container mx-auto px-4 py-8">
        {/* Summary Cards */}
        <section className="mb-8">
          <StatCards stats={stats} />
        </section>

        {/* Keyboard Heatmap */}
        <section className="mb-8">
          <Card>
            <CardHeader>
              <CardTitle className="text-lg">⌨️ Keyboard Heatmap</CardTitle>
            </CardHeader>
            <CardContent className="overflow-x-auto">
              <KeyboardHeatmap stats={stats} />
            </CardContent>
          </Card>
        </section>

        {/* Charts Grid */}
        <section className="mb-8 grid grid-cols-1 lg:grid-cols-2 gap-6">
          <TopKeysChart stats={stats} />
          <TopCombosChart stats={stats} />
          <HourlyActivityChart stats={stats} />
          <DailyActivityChart stats={stats} />
          <KeyTypesPieChart stats={stats} />
          <SpecialKeysChart stats={stats} />
        </section>

        {/* Shareable Image */}
        <section className="mb-8">
          <ShareableImage stats={stats} />
        </section>

        {/* Additional Stats */}
        <section className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <HighlightsCard stats={stats} />
          <RecordingInfoCard stats={stats} />
        </section>
      </main>

      {/* Footer */}
      <footer className="border-t border-border py-6 mt-8 bg-card">
        <div className="container mx-auto px-4 text-center text-muted-foreground text-sm">
          <p>
            Created with ❤️ by{' '}
            <a
              href="https://twlite.dev?utm_source=kitmap"
              target="_blank"
              rel="noopener noreferrer"
              className="text-primary font-semibold"
            >
              Twilight
            </a>
          </p>
        </div>
      </footer>
    </div>
  );
}

export default App;
