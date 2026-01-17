export interface KeyStats {
  key_name: string;
  count: number;
  percentage: number;
}

export interface ComboStats {
  combo: string;
  count: number;
}

export interface HourlyStats {
  hour: number;
  count: number;
}

export interface DailyStats {
  day: string;
  count: number;
}

export interface AllStats {
  total_keys: number;
  total_combos: number;
  total_sessions: number;
  total_time_minutes: number;
  most_pressed_key: KeyStats | null;
  most_pressed_combo: ComboStats | null;
  top_keys: KeyStats[];
  top_combos: ComboStats[];
  spacebar_count: number;
  enter_count: number;
  backspace_count: number;
  delete_count: number;
  escape_count: number;
  tab_count: number;
  arrow_keys_count: number;
  modifier_keys_count: number;
  letter_keys_count: number;
  number_keys_count: number;
  special_keys_count: number;
  hourly_distribution: HourlyStats[];
  daily_distribution: DailyStats[];
  most_active_hour: HourlyStats | null;
  most_active_day: DailyStats | null;
  average_keys_per_session: number;
  average_typing_speed: number;
  max_typing_speed: number;
  key_frequency_map: Record<string, number>;
  first_recorded: string | null;
  last_recorded: string | null;
  unique_keys_used: number;
  keys_per_minute_avg: number;
}
